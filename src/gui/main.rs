#![allow(non_snake_case)]
use std::collections::HashMap;
use std::sync::atomic::Ordering;

use clap::ValueEnum;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use itertools::Itertools;
use perk_solver::{prelude::*, Solver, SolverMetadata};
use strum::VariantNames;

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
    let solver = use_state(cx, || None::<SolverMetadata>);
    let error = use_state(cx, || None);
    let result = use_state(cx, || None);

    cx.render(rsx!(
        style { include_str!("./css/bootstrap.min.css") },
        style { include_str!("./css/common.css") },
        ArgsForm {
            on_submit: move |ev: FormEvent| {
                println!("Sumitted: {:?}", ev.values);
                if let Some(s) = solver.get() {
                    s.cancel_signal.store(true, Ordering::Relaxed);
                    solver.set(None);
                } else {
                    match form_to_args(&ev.values) {
                        Ok(args) => {
                            match Solver::new(args, Data::load()) {
                                Ok(s) => {
                                    error.set(None);
                                    result.set(None);
                                    solver.set(Some(s.meta.clone()));

                                    cx.spawn({
                                        let result = result.to_owned();
                                        async move {
                                            result.set(Some(s.run().join().unwrap()));
                                        }
                                    });
                                }
                                Err(err) => {
                                    error.set(Some(err));
                                    result.set(None);
                                }
                            }
                        }
                        Err(err) => {
                            error.set(Some(err));
                            result.set(None);
                        }
                    }
                }
            },
            is_running: solver.get().is_some() && result.get().is_none()
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
        if let Some(solver) = solver.get() {
            rsx!(
                MaterialsList(cx, solver),

            )
        }
    ))
}

fn MaterialsList<'a>(cx: Scope<'a>, solver: &'a SolverMetadata) -> Element<'a> {
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
                        solver.materials.conflict
                            .iter()
                            .chain(&solver.materials.no_conflict)
                            .sorted()
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
                                class: "form-control",
                                for &x in PerkName::VARIANTS.iter() {
                                    option { value: x, x }
                                }
                            }
                            input { r#type: "number", name: "rank one", placeholder: "Rank", min: "1", max: "6", class: "form-control" }
                        }
                    }
                    tr {
                        th { "Perk two:" }
                        td {
                            select {
                                name: "perk two",
                                class: "form-control",
                                for &x in ["Any"].iter().chain(PerkName::VARIANTS) {
                                    option { value: x, x }
                                }
                            }
                            input { r#type: "number", name: "rank two", placeholder: "Rank", min: "1", max: "6", class: "form-control" }
                        }
                    }
                    tr {
                        th { "Invention level:" }
                        td {
                            input { r#type: "number", name: "invention level low", min: "1", max: "137", value: "1", class: "form-control" }
                            span { "—" }
                            input { r#type: "number", name: "invention level heigh", min: "1", max: "137", value: "137", class: "form-control" }
                        }
                    }
                    tr {
                        th { "Ancient:" }
                        td {
                            input { r#type: "checkbox", name: "ancient", class: "form-control" }
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
                                input { r#type: "radio", name: "gizmo type", id: x, value: x, class: "form-control" }
                                label { r#for: x, x }
                            }
                        }
                    }
                    tr {
                        th { "Alt count:" }
                        td {
                            input { r#type: "number", name: "alt count", min: "0", max: "254", value: "5", class: "form-control" }
                        }
                    }

                }
                table {
                    tr {
                        th { "Sort on:" }
                    }
                    for x in ["Lowest price", "Best gizmo chance", "Best attempt chance"] {
                        tr {
                            td {
                                input {
                                    r#type: "radio",
                                    name: "sort type",
                                    id: x,
                                    value: x,
                                    checked: if x == "Lowest price" { "true" } else { "false" },
                                    class: "form-control"
                                }
                                label { r#for: x, x }
                            }
                        }
                    }
                    tr {
                        th { "Exclude filter:" }
                    }
                    tr {
                        td {
                            input { r#type: "text", name: "exclude filter", placeholder: "e.g.: noxious, direct", class: "form-control" }
                        }
                    }
                }
            }

            button { r#type: "submit", value: "Submit", class: "form-control", if *is_running { "Cancel" } else { "Start!" } }
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
            out_file: String::new(),
            price_file: String::new(),
            alt_count,
        },
    };
    Args::create(&cli)
}
