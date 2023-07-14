use clap::ValueEnum;
use dioxus::prelude::*;
use perk_solver::prelude::*;
use std::collections::HashMap;
use strum::VariantNames;

#[inline_props]
pub fn ArgsForm<'a>(
    cx: Scope,
    on_submit: EventHandler<'a, FormEvent>,
    is_running: bool,
) -> Element {
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
                            input { r#type: "number", name: "invention level high", min: "1", max: "137", value: "137", margin_left: "3px" }
                        }
                    }
                    tr {
                        th { "Ancient:" }
                        td {
                            input { r#type: "checkbox", name: "ancient", checked: "true" }
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
                table {
                    tr {
                        th {
                            class: "help",
                            title: "Amount of additional unique material combination to show after the best one.",
                            "Alt count:"
                        }
                        td {
                            input { r#type: "number", name: "alt count", min: "0", max: "254", value: "5" }
                        }
                    }
                    tr {
                        th {
                            class: "help",
                            title: "Comma separated list of material values to exclude. Uses basic substring matching.",
                            "Exclude filter:"
                        }
                        td {
                            input { r#type: "text", name: "exclude filter", placeholder: "e.g.: noxious, direct" }
                        }
                    }
                    tr {
                        th {
                            class: "help",
                            title: "Limit the number of threads used to 80% of the amount available on the system rounded down.",
                            "Limit CPU usage:"
                        }
                        td {
                            input { r#type: "checkbox", name: "limit CPU", checked: "false" }
                        }
                    }
                }
            }
            if *is_running {
                rsx!(
                    button { r#type: "submit", value: "Submit", class: "btn btn-danger" , "Cancel" }
                )
            } else {
                rsx!(
                    button { r#type: "submit", value: "Submit", class: "btn btn-primary", "Start!" }
                )
            }
        }
    ))
}

pub fn form_to_args(values: &HashMap<String, String>) -> Result<Args, String> {
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
            limit_cpu: values.get("limit CPU").unwrap() == "true",
        },
    };
    Args::create(&cli)
}
