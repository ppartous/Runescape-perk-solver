use crate::{prelude::*, result::gizmo_combination_sort, component_prices::load_component_prices};
use std::time::Duration;
use std::ffi::{c_char, CStr, CString};
use std::sync::{Mutex, Arc, atomic::AtomicBool, atomic::Ordering};
use itertools::Itertools;
use tokio::time;

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
    pub price_file: *const c_char
}

#[derive(Debug)]
#[repr(C)]
pub struct Response {
    bar_progress: *const u64,
    total_combination_count: usize,
}

// Any pointer previously obtained from `get_result_json` become dangling after calling this function.
#[tokio::main]
#[no_mangle]
pub async extern "C" fn perk_solver_ctypes(args: FfiArgs) -> Response {
    let mut perk = String::new();
    let mut perk_two = String::new();
    let mut exclude = Vec::new();
    let mut out_file = Args::default().out_file;
    let mut price_file = Args::default().price_file;

    unsafe {
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
            price_file
        }
    };

    let args = Args::create(&cli);
    let data = Data::load();
    let wanted_gizmo = Gizmo {
        perks: (
            Perk { name: args.perk, rank: args.rank },
            Perk { name: args.perk_two, rank: args.rank_two }
        ),
        ..Default::default()
    };
    let s = crate::setup(args, &data, wanted_gizmo);
    let ptr = Arc::into_raw(s.bar_progress.clone()) as *const u64;
    let has_started = Arc::new(AtomicBool::new(false));

    load_component_prices(&s.args);

    {
        let x = has_started.clone();
        std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    x.store(true, Ordering::SeqCst);
                    let mut channel = CHANNEL.lock().unwrap();
                    crate::perk_solver_core(s.args, data, wanted_gizmo, s.materials, s.bar_progress, s.total_combination_count, s.result_tx).await;
                    let mut best_per_level = s.result_handler.join().unwrap();
                    for x in best_per_level.iter_mut() {
                        x.mat_combination = Arc::new(gizmo_combination_sort(&x.mat_combination));
                    }
                    let json = serde_json::to_string(&best_per_level).unwrap();
                    channel.replace(CString::new(json).unwrap());
                })
        });
    }

    let mut interval = time::interval(Duration::from_millis(10));
    while !has_started.load(Ordering::SeqCst) {
        interval.tick().await;
    }

    Response {
        bar_progress: ptr,
        total_combination_count: s.total_combination_count
    }
}

#[no_mangle]
pub extern "C" fn get_result_json() -> *const c_char {
    let json = CHANNEL.lock().unwrap();

    if let Some(s) = json.as_ref() {
        s.as_ptr()
    } else  {
        std::ptr::null()
    }
}