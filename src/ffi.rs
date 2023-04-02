use crate::{prelude::*, result::gizmo_combination_sort, component_prices::load_component_prices};
use std::ffi::{c_char, CStr, CString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, Arc, Condvar};
use itertools::Itertools;

static CHANNEL: Mutex<Option<CString>> = Mutex::new(None);

#[derive(Debug)]
#[repr(C)]
pub struct FfiArgs {
    pub ancient: bool,
    pub gizmo_type: GizmoType,
    pub invention_level: [u8; 2],
    pub perk: *const c_char,
    pub rank: u8,
    pub perk_two: *const c_char,
    pub rank_two: u8,
    pub fuzzy: bool,
    pub exclude: *const c_char,
    pub sort_type: SortType,
    pub out_file: *const c_char,
    pub price_file: *const c_char,
    pub alt_count: u8
}

#[repr(C)]
pub struct Response {
    total_combination_count: usize,
    bar_progress: *const u64,
    materials: *const c_char,
    result: *const c_char,
    cancel_signal: *const AtomicBool
}

impl From<String> for Response {
    fn from(value: String) -> Self {
        Response {
            total_combination_count: 0,
            bar_progress: std::ptr::null(),
            materials: std::ptr::null(),
            result: CString::new(value).unwrap().into_raw() as *const c_char,
            cancel_signal: std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn perk_solver_ctypes(args: FfiArgs) -> Response {
    let mut perk = String::new();
    let mut perk_two = String::new();
    let mut exclude = Vec::new();
    let mut out_file = Args::default().out_file;
    let mut price_file = Args::default().price_file;

    if !args.perk.is_null() {
        perk = CStr::from_ptr(args.perk).to_str().unwrap().to_string();
    }
    if !args.perk_two.is_null() {
        perk_two = CStr::from_ptr(args.perk_two).to_str().unwrap().to_string();
    }
    if !args.out_file.is_null() {
        out_file = CStr::from_ptr(args.out_file).to_str().unwrap().to_string();
    }
    if !args.price_file.is_null() {
        price_file = CStr::from_ptr(args.price_file).to_str().unwrap().to_string();
    }
    if !args.exclude.is_null() {
        exclude = CStr::from_ptr(args.exclude).to_str().unwrap().split(",").map(|x| x.to_string()).collect_vec();
    }

    let cli = Cli {
        ancient: args.ancient,
        gizmo_type: args.gizmo_type,
        invention_level: args.invention_level.to_vec(),
        command: Commands::Gizmo {
            perk,
            rank: args.rank,
            perk_two: if perk_two.is_empty() { None } else { Some(perk_two) },
            rank_two: args.rank_two,
            fuzzy: args.fuzzy,
            exclude,
            sort_type: args.sort_type,
            out_file,
            price_file,
            alt_count: args.alt_count
        }
    };

    colored::control::set_override(false); // Disable colored messages

    let data = Data::load();
    let args = match Args::create(&cli) {
        Ok(args) => args,
        Err(err) => return Response::from(err)
    };
    let s = match crate::setup(args, &data) {
        Ok(s) => s,
        Err(err) => return Response::from(err)
    };
    if let Err(err) = load_component_prices(&s.args) {
        return Response::from(err);
    }

    let bar_progress = Arc::into_raw(s.bar_progress.clone()) as *const u64;
    let materials = CString::new(s.materials.to_json()).unwrap();
    let total_combination_count = s.total_combination_count;
    let cancel_signal = Arc::into_raw(s.cancel_signal.clone());
    let has_started = Arc::new((Mutex::new(false), Condvar::new()));
    let has_started2 = Arc::clone(&has_started);

    std::thread::spawn(move || {
        let mut channel = CHANNEL.lock().unwrap();
        let (lock, cvar) = &*has_started2;
        if let Ok(mut started) = lock.lock() {
            *started = true;
            cvar.notify_one();
        }

        crate::perk_solver_core(data, &s);
        let mut best_per_level = s.result_handler.join().unwrap()
            .into_iter().map(|x| x.into_iter().filter(|y| y.prob_gizmo > 0.0).collect_vec()).collect_vec();
        for x in best_per_level.iter_mut() {
            for y in x.iter_mut() {
                y.mat_combination = Arc::new(gizmo_combination_sort(&y.mat_combination));
            }
        }
        let json = serde_json::to_string(&best_per_level).unwrap();
        channel.replace(CString::new(json).unwrap());
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*has_started;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    Response {
        total_combination_count,
        bar_progress,
        materials: materials.into_raw(),
        result: std::ptr::null(),
        cancel_signal
    }
}

#[no_mangle]
pub extern "C" fn get_result(response: &mut Response) {
    let json = CHANNEL.lock().unwrap().take();

    if let Some(s) = json {
        response.result = s.into_raw();
    }
}

#[no_mangle]
pub unsafe extern "C" fn cancel_and_free(mut response: Response) {
    if response.cancel_signal != std::ptr::null() {
        let cancel_signal = Arc::from_raw(response.cancel_signal);
        response.cancel_signal = std::ptr::null();
        cancel_signal.store(true, Ordering::SeqCst);
        get_result(&mut response); // Wait untill everything has stopped
        free_response(response);
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_response(response: Response) {
    if response.bar_progress != std::ptr::null() {
        let bar_progress = Arc::from_raw(response.bar_progress);
        drop(bar_progress);
    }

    if response.cancel_signal != std::ptr::null() {
        let cancel_signal = Arc::from_raw(response.cancel_signal);
        drop(cancel_signal);
    }

    if response.result != std::ptr::null() {
        let result = CString::from_raw(response.result as *mut c_char);
        drop(result);
    }

    if response.materials != std::ptr::null() {
        let materials = CString::from_raw(response.materials as *mut c_char);
        drop(materials);
    }
}