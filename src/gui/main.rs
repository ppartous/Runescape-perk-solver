#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Prevents opening a console window alongside the gui
mod args;
mod prices_tab;
mod result;
mod wiki;

use std::sync::atomic::Ordering;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_desktop::tao::window::Icon;
use dioxus_desktop::{Config, WindowBuilder};

use indicatif::HumanCount;
use itertools::Itertools;
use perk_solver::{component_prices, prelude::*, Solver, SolverMetadata};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use strum_macros::EnumVariantNames;
use tokio::time;
use wiki::WikiImage;

fn main() {
    colored::control::set_override(false); // Disable colored error message strings
    let icon = Icon::from_rgba(include_bytes!("../../images/icon.bin").to_vec(), 32, 32).unwrap();

    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title(format!("Perk solver {}", env!("CARGO_PKG_VERSION")))
            .with_inner_size(dioxus_desktop::LogicalSize::new(1100.0, 950.0))
            .with_resizable(true)
            .with_window_icon(Some(icon)),
    );

    dioxus_desktop::launch_cfg(App, config);
}

fn App(cx: Scope) -> Element {
    let solver = use_ref(cx, || None::<SolverMetadata>);
    let error = use_state(cx, || None);
    let result = use_ref(cx, || None);
    let progress = use_state(cx, || 0);
    let start_time = use_state(cx, || None);
    let end_time = use_state(cx, || None);
    let tab_selection = use_state(cx, || TabSelection::Result);
    let prices_status = use_state(cx, || None::<Result<component_prices::PriceSource, String>>);

    let on_submit = move |ev: FormEvent| {
        if solver.read().is_some() && result.read().is_none() {
            solver
                .write()
                .as_ref()
                .unwrap()
                .cancel_signal
                .store(true, Ordering::Relaxed);
        } else {
            *result.write() = None;
            *solver.write() = None;
            start_time.set(None);
            end_time.set(None);
            error.set(None);

            match args::form_to_args(&ev.values) {
                Ok(args) => match Solver::new(args, Data::load()) {
                    Ok(s) => {
                        *solver.write() = Some(s.meta.clone());
                        progress.set(0);
                        start_time.set(Some(time::Instant::now()));

                        cx.spawn({
                            to_owned![result, end_time];
                            async move {
                                let res = tokio::task::spawn_blocking(move || s.run()).await;
                                *result.write() = Some(res.unwrap());
                                end_time.set(Some(time::Instant::now()));
                            }
                        });

                        cx.spawn({
                            to_owned![progress, solver, result];
                            async move {
                                let mut interval = time::interval(Duration::from_millis(200));
                                loop {
                                    interval.tick().await;
                                    if let Some(solver) = solver.read().as_ref() {
                                        let val = solver.bar_progress.load(Ordering::Relaxed);
                                        progress.set(val);
                                        if val == solver.total_combination_count {
                                            break;
                                        }
                                    }
                                    if result.read().is_some() {
                                        break;
                                    }
                                }
                            }
                        })
                    }
                    Err(err) => {
                        error.set(Some(err));
                    }
                },
                Err(err) => {
                    error.set(Some(err));
                }
            }
        }
    };

    use_future(cx, prices_status, |prices_status| async move {
        if prices_status.get().is_none() {
            let res = tokio::task::spawn_blocking(|| {
                perk_solver::component_prices::load_component_prices(
                    &Args::default().price_file,
                    true,
                )
            })
            .await;
            prices_status.set(Some(res.unwrap()));
        }
    });

    cx.render(rsx!(
        style { include_str!("./css/common.css") },
        args::ArgsForm {
            on_submit: on_submit,
            is_running: solver.read().is_some() && result.read().is_none()
        },
        div {
            class: "tabber",
            for x in TabSelection::iter() {
                button {
                    onclick: move |_| tab_selection.set(x),
                    class: if *tab_selection.get() == x { "tab is-selected"} else { "tab" },
                    Into::<&'static str>::into(x)
                }
            }
        }
        div {
            id: "content",
            match tab_selection.get() {
                TabSelection::Result => rsx!(
                    if let Some(err) = error.get() {
                        rsx!(
                            div {
                                class: "error",
                                b { "Error: " },
                                err.clone()
                            }
                        )
                    }
                    if let Some(solver) = solver.read().as_ref() {
                        rsx!(
                            MaterialsList(cx, solver),
                            ProgressBar {
                                val: *progress.get(),
                                max: solver.total_combination_count,
                                is_running: result.read().is_none(),
                                ellapsed: if let Some(end_t) = end_time.get() {
                                    end_t.duration_since(start_time.get().unwrap())
                                } else {
                                    start_time.get().unwrap().elapsed()
                                }
                            }
                            if let Some(result) = result.read().as_ref() {
                                rsx!(
                                    result::ResultTable(cx, result, &solver.args)
                                )
                            }
                        )
                    }
                ),
                TabSelection::FullResult => rsx!(
                    if let Some(result) = result.read().as_ref() {
                        rsx!(result::FullResultTable(cx, result))
                    }
                ),
                TabSelection::Prices => rsx!(
                    prices_tab::PricesTab(cx, &prices_status)
                )
            }
        }
    ))
}

#[derive(EnumIter, EnumVariantNames, IntoStaticStr, Clone, Copy, PartialEq, Eq)]
enum TabSelection {
    Result,
    #[strum(serialize = "Full result")]
    FullResult,
    Prices,
}

#[inline_props]
fn ProgressBar(
    cx: Scope,
    val: u64,
    max: u64,
    is_running: bool,
    ellapsed: time::Duration,
) -> Element<'a> {
    let width_val = *val as f64 / *max as f64 * 100.0;
    let val = HumanCount(*val);
    let max = HumanCount(*max);
    let hours = ellapsed.as_secs() / 3600;
    let mins = (ellapsed.as_secs() / 60) % 60;
    let secs = ellapsed.as_secs() % 60;

    cx.render(rsx!(
        div {
            class: "progressbar-wrapper",
            div {
                class: "progressbar",
                div {
                    class: if *is_running { "progressbar-bg progress-bar-animated" } else {"progressbar-bg"},
                    width: "{width_val}%",
                }
            }
            div {
                class: "progressbar-text",
                div {
                    "{hours:02}:{mins:02}:{secs:02}"
                }
                div {
                    "{val} / {max} ({width_val:.0}%)"
                }
            }
        }
    ))
}

fn MaterialsList<'a>(cx: Scope<'a>, solver: &SolverMetadata) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "materials",
            b { "Materials: " }
            span {
                for (i, mat) in solver.materials.conflict.iter().chain(&solver.materials.no_conflict).sorted().enumerate() {
                    if i > 0 {
                        rsx!(", ")
                    }
                    WikiImage(cx, mat.to_str())
                    "{mat}"
                }
            }
        }
        if !solver.args.exclude.is_empty() {
            rsx!(
                div {
                    class: "excluded-materials",
                    b { "Excluded materials: " }
                    span {
                        for (i, mat) in solver.args.exclude.iter().enumerate() {
                            if i > 0 {
                                rsx!(", ")
                            }
                            WikiImage(cx, mat.to_str())
                            "{mat}"
                        }
                    }
                }
            )
        }
    ))
}
