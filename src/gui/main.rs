#![allow(non_snake_case)]
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::time::Duration;

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use clap::ValueEnum;
use indicatif::HumanCount;
use itertools::Itertools;
use perk_solver::{prelude::*, Solver, SolverMetadata};
use strum::VariantNames;
use tokio::time;

fn main() {
    colored::control::set_override(false); // Disable colored messages
    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title(format!("Perk solver {}", env!("CARGO_PKG_VERSION")))
            .with_inner_size(dioxus_desktop::LogicalSize::new(800.0, 500.0))
            .with_resizable(true),
    );

    dioxus_desktop::launch_cfg(App, config);
}

fn App(cx: Scope) -> Element {
    let solver = use_ref(cx, || None::<SolverMetadata>);
    let error = use_state(cx, || None);
    let result = use_ref(cx, || None);
    let progress = use_state(cx, || 0);

    let on_submit = move |ev: FormEvent| {
        println!("Sumitted: {:?}", ev.values);
        if solver.read().is_some() && result.read().is_none() {
            solver
                .write()
                .as_ref()
                .unwrap()
                .cancel_signal
                .store(true, Ordering::Relaxed);
            println!("Canceling");
        } else {
            *result.write() = None;
            match form_to_args(&ev.values) {
                Ok(args) => {
                    error.set(None);
                    match Solver::new(args, Data::load()) {
                        Ok(s) => {
                            *solver.write() = Some(s.meta.clone());

                            cx.spawn({
                                to_owned![result];
                                async move {
                                    let res = tokio::task::spawn_blocking(move || s.run()).await;
                                    *result.write() = Some(res.unwrap());
                                }
                            });

                            cx.spawn({
                                to_owned![progress, solver, result];
                                async move {
                                    let mut interval = time::interval(Duration::from_millis(200));
                                    loop {
                                        interval.tick().await;
                                        if result.read().is_some() {
                                            break;
                                        } else if let Some(solver) = solver.read().as_ref() {
                                            let val = solver.bar_progress.load(Ordering::Relaxed);
                                            progress.set(val);
                                            if val == solver.total_combination_count {
                                                break;
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            })
                        }
                        Err(err) => {
                            error.set(Some(err));
                        }
                    }
                }
                Err(err) => {
                    error.set(Some(err));
                }
            }
        }
    };

    cx.render(rsx!(
        style { include_str!("./css/common.css") },
        ArgsForm {
            on_submit: on_submit,
            is_running: solver.read().is_some() && result.read().is_none()
        },
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
                    max: solver.total_combination_count
                }
            )
        }
        if let Some(result) = result.read().as_ref() {
            rsx!(
                div {
                    format!("{result:?}")
                }
            )
        }
    ))
}

#[inline_props]
fn ProgressBar(cx: Scope, val: u64, max: u64) -> Element {
    let width_val = *val as f64 / *max as f64 * 100.0;
    let val = HumanCount(*val);
    let max = HumanCount(*max);

    cx.render(rsx!(
        div {
            class: "progressbar-wrapper",
            div {
                class: "progressbar",
                div {
                    class: "progressbar-bg",
                    width: "{width_val}%",
                }
            }
            div {
                class: "progressbar-text",
                "{val} / {max} ({width_val:.0}%)"
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
                solver.materials.conflict
                    .iter()
                    .chain(&solver.materials.no_conflict)
                    .sorted()
                    .map(|x| x.to_string())
                    .join(", ")
            }
        }
        if !solver.args.exclude.is_empty() {
            rsx!(
                div {
                    class: "excluded-materials",
                    b { "Excluded materials: " }
                    span {
                        solver.args.exclude
                            .iter()
                            .map(|x| x.to_string())
                            .join(", ")
                    }
                }
            )
        }
    ))
}

#[inline_props]
fn ArgsForm<'a>(cx: Scope, on_submit: EventHandler<'a, FormEvent>, is_running: bool) -> Element {
    cx.render(rsx!(
        form {
            id: "ArgsForm",
            onsubmit: move |ev| on_submit.call(ev),
            div {
                table {
                    tr {
                        th { "Perk one:" }
                        td {
                            select {
                                name: "perk one",
                                margin_right: "3px",
                                for &x in PerkName::VARIANTS.iter() {
                                    option { value: x, x }
                                }
                            }
                            input { r#type: "number", name: "rank one", placeholder: "Rank", min: "1", max: "6" }
                        }
                    }
                    tr {
                        th { "Perk two:" }
                        td {
                            select {
                                name: "perk two",
                                margin_right: "3px",
                                for &x in ["Any"].iter().chain(PerkName::VARIANTS) {
                                    option { value: x, x }
                                }
                            }
                            input { r#type: "number", name: "rank two", placeholder: "Rank", min: "1", max: "6" }
                        }
                    }
                    tr {
                        th { "Invention level:" }
                        td {
                            input { r#type: "number", name: "invention level low", min: "1", max: "137", value: "1", margin_right: "3px" }
                            span { "—" }
                            input { r#type: "number", name: "invention level heigh", min: "1", max: "137", value: "137", margin_left: "3px" }
                        }
                    }
                    tr {
                        th { "Ancient:" }
                        td {
                            input { r#type: "checkbox", name: "ancient", checked: "true" }
                        }
                    }
                    tr {
                        th { "Exclude filter:" }
                        td {
                            input { r#type: "text", name: "exclude filter", placeholder: "e.g.: noxious, direct" }
                        }
                    }
                }
                table {
                    tr {
                        th {
                            colspan: "2",
                            "Gizmo type:"
                        }
                    }
                    for x in ["Weapon", "Armour", "Tool"] {
                        tr {
                            td {
                                colspan: "2",
                                input { r#type: "radio", name: "gizmo type", id: x, value: x, class: "form-check-input" }
                                label { r#for: x, x }
                            }
                        }
                    }
                    tr {
                        th { "Alt count:" }
                        td {
                            input { r#type: "number", name: "alt count", min: "0", max: "254", value: "5" }
                        }
                    }

                }
                table {
                    tr {
                        th { "Sort on:" }
                    }
                    for x in [("Lowest price", "Price"), ("Best gizmo chance", "Gizmo"), ("Best attempt chance", "Attempt")] {
                        tr {
                            td {
                                input {
                                    r#type: "radio",
                                    name: "sort type",
                                    id: x.0,
                                    value: x.1,
                                    checked: if x.1 == "Price" { "true" } else { "false" },
                                }
                                label { r#for: x.0, x.0 }
                            }
                        }
                    }
                }
            }
            if *is_running {
                rsx!(
                    button { r#type: "submit", value: "Submit", class: "btn-danger" , "Cancel" }
                )
            } else {
                rsx!(
                    button { r#type: "submit", value: "Submit", class: "btn-primary", "Start!" }
                )
            }
        }
    ))
}

fn form_to_args(values: &HashMap<String, String>) -> Result<Args, String> {
    let gizmo_type = GizmoType::from_str(
        values
            .get("gizmo type")
            .ok_or(String::from("Please select a gizmo type"))?,
        true,
    )?;
    let mut invention_level = vec![];
    if let Some(x) = values.get("invention level low") {
        if !x.is_empty() {
            invention_level.push(x.parse().unwrap());
        }
    }
    if let Some(x) = values.get("invention level high") {
        if !x.is_empty() {
            invention_level.push(x.parse().unwrap());
        }
    }
    let rank = values
        .get("rank one")
        .map(|x| x.parse().unwrap_or(1))
        .unwrap_or(1);
    let rank_two = values
        .get("rank two")
        .map(|x| x.parse().unwrap_or(1))
        .unwrap_or(1);
    let exclude = values
        .get("exclude filter")
        .map(|x| x.replace(", ", ",").split(",").map(String::from).collect())
        .unwrap_or_default();
    let sort_type = SortType::from_str(
        values
            .get("sort type")
            .ok_or(String::from("Please select a sort type"))?,
        true,
    )?;
    let alt_count = values
        .get("alt count")
        .map(|x| x.parse().unwrap_or(0))
        .unwrap_or(0);

    let cli = Cli {
        ancient: values.get("ancient").unwrap() == "true",
        gizmo_type,
        invention_level,
        command: Commands::Gizmo {
            perk: values.get("perk one").unwrap().clone(),
            rank,
            perk_two: values.get("perk two").map(String::from),
            rank_two,
            fuzzy: false,
            exclude,
            sort_type,
            out_file: String::from("false"),
            price_file: String::from("false"),
            alt_count,
        },
    };
    Args::create(&cli)
}
