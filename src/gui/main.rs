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
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr, VariantNames};
use strum_macros::EnumVariantNames;
use tokio::time;

fn main() {
    colored::control::set_override(false); // Disable colored messages
    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title(format!("Perk solver {}", env!("CARGO_PKG_VERSION")))
            .with_inner_size(dioxus_desktop::LogicalSize::new(950.0, 950.0))
            .with_resizable(true),
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
    let prices_status = use_state(cx, || None::<Result<(), String>>);

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

            match form_to_args(&ev.values) {
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

    use_future(cx, (), |_| {
        to_owned![prices_status];
        async move {
            let res = tokio::task::spawn_blocking(|| {
                perk_solver::component_prices::load_component_prices("false")
            })
            .await;
            prices_status.set(Some(res.unwrap()));
        }
    });

    cx.render(rsx!(
        style { include_str!("./css/common.css") },
        ArgsForm {
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
                                    ResultTable(cx, result, &solver.args)
                                )
                            }
                        )
                    }
                ),
                TabSelection::FullResult => rsx!(
                    if let Some(result) = result.read().as_ref() {
                        rsx!(FullResultTable(cx, result))
                    }
                ),
                TabSelection::Prices => rsx!(
                    PricesTab(cx, prices_status.get())
                    // None::<Element>
                )
            }
        }
    ))
}

fn PricesTab<'a>(cx: Scope<'a>, prices_status: &Option<Result<(), String>>) -> Element<'a> {
    cx.render(rsx!(if let Some(status) = prices_status {
        rsx!(if let Err(err) = status {
            rsx!("{err}")
        } else {
            rsx!(
                h3 { "Common materials" }
                div {
                    class: "prices-container",
                    table {
                        class: "prices-table",
                        for mat in COMMON_MATERIALS.iter() {
                            PriceTabElement(cx, *mat)
                        }
                    }
                }
                h3 { "Uncommon materials" }
                div {
                    class: "prices-container",
                    table {
                        class: "prices-table",
                        for mat in UNCOMMON_MATERIALS.iter() {
                            PriceTabElement(cx, *mat)
                        }
                    }
                }
                h3 { "Rare materials" }
                div{
                    class: "prices-container",
                    table {
                        class: "prices-table",
                        for mat in RARE_MATERIALS.iter() {
                            PriceTabElement(cx, *mat)
                        }
                    }
                }
            )
        })
    } else {
        rsx!("Fetching component prices from Runescape.wiki...")
    }))
}

fn PriceTabElement(cx: Scope, mat: MaterialName) -> Element {
    cx.render(rsx!(
        tr {
            td {
                img {
                    src: "https://runescape.wiki/images/{mat.to_string().replace(\" \", \"_\")}.png?00000",
                }
                "{mat}:"
            }
            td {
                input {
                    r#type: "number",
                    min: "0",
                    value: *perk_solver::component_prices::PRICES.read().unwrap().as_ref().unwrap().get(mat),
                    oninput: move |ev| {
                        if let Some(prices) = perk_solver::component_prices::PRICES.write().unwrap().as_mut() {
                            *prices.get_mut(mat) = ev.value.parse().unwrap_or(0.0);
                        }
                    }
                }
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

fn get_color(val: f64) -> (u8, u8, u8) {
    let val = val.powi(10);
    (
        ((1.0 - val.clamp(0.5, 1.0)) * 2.0 * 190.0) as u8,
        (val.clamp(0.0, 0.5) * 2.0 * 190.0) as u8,
        0,
    )
}

fn FullResultTable<'a>(cx: Scope<'a>, result: &Vec<Vec<ResultLine>>) -> Element<'a> {
    cx.render(rsx!(
        table {
            class: "result-table",
            tr {
                th { rowspan: 2, "Level" }
                th { colspan: 2, "Probability (%)" }
                th { rowspan: 2, "Price" }
                th { rowspan: 2, "Material combination" }
            }
            tr {
                th { "Gizmo" }
                th { "Attempt" }
            }
            for lines in result.iter() {
                tr {
                    td { rowspan: "{lines.len()}", "{lines[0].level}" }
                    td { perk_solver::result::format_float(lines[0].prob_gizmo) }
                    td { perk_solver::result::format_float(lines[0].prob_attempt) }
                    td { perk_solver::result::format_price(lines[0].price) }
                    td { MaterialName::vec_to_string(&lines[0].mat_combination) }
                }
                for line in lines.iter().skip(1) {
                    tr {
                        td { perk_solver::result::format_float(line.prob_gizmo) }
                        td { perk_solver::result::format_float(line.prob_attempt) }
                        td { perk_solver::result::format_price(line.price) }
                        td { MaterialName::vec_to_string(&line.mat_combination) }
                    }
                }
            }
        }
    ))
}

fn ResultTable<'a>(cx: Scope<'a>, result: &Vec<Vec<ResultLine>>, args: &Args) -> Element<'a> {
    if let Some((best_gizmo_index, best_attempt_index, best_price_index)) =
        perk_solver::result::get_best_of_each(result)
    {
        let best_wanted_index = match args.sort_type {
            SortType::Gizmo => best_gizmo_index,
            SortType::Attempt => best_attempt_index,
            SortType::Price => best_price_index,
        };

        let best_wanted = &result[best_wanted_index][0];
        let best_gizmo_prob = result[best_gizmo_index][0].prob_gizmo;
        let best_attempt_prob = result[best_attempt_index][0].prob_attempt;
        let best_price = result[best_price_index][0].price;

        cx.render(rsx!(
            div {
                class: "result",
                table {
                    class: "result-table",
                    tr {
                        th { rowspan: 2, "Level" }
                        th { colspan: 2, "Probability (%)" }
                        th { rowspan: 2, "Price" }
                    }
                    tr {
                        th { "Gizmo" }
                        th { "Attempt" }
                    }
                    for (line, (r1, g1, _), (r2, g2, _), (r3, g3, _)) in result
                        .iter()
                        .map(|x| (
                            x,
                            get_color(x[0].prob_gizmo / best_gizmo_prob),
                            get_color(x[0].prob_attempt / best_attempt_prob),
                            get_color(best_price / x[0].price),
                        ))
                    {
                        tr {
                            td {
                                match line[0].level - (line[0].level % 2) {
                                    0 => String::from("1"),
                                    x => format!("{x}-{}", x + 1)
                                }
                            }
                            td {
                                background_color: "rgba({r1},{g1},0, 0.5)",
                                perk_solver::result::format_float(line[0].prob_gizmo)
                            }
                            td {
                                background_color: "rgba({r2},{g2},0, 0.5)",
                                perk_solver::result::format_float(line[0].prob_attempt)
                            }
                            td {
                                background_color: "rgba({r3},{g3},0, 0.5)",
                                perk_solver::result::format_price(line[0].price)
                            }
                        }
                    }
                }
                div {
                    table {
                        class: "result-table result-alts",
                        tr {
                            th {
                                match args.sort_type {
                                    SortType::Gizmo => "Prob. per gizmo",
                                    SortType::Attempt => "Prob. per attempt",
                                    SortType::Price => "Price"
                                }
                            }
                            th { "Level" }
                            th { "Material combination" }
                        }
                        tr {
                            th { colspan: 3, padding: "0", line_height: "1.5em", "Best" }
                        }
                        tr {
                            td {
                                match args.sort_type {
                                    SortType::Gizmo => perk_solver::result::format_float(best_wanted.prob_gizmo),
                                    SortType::Attempt => perk_solver::result::format_float(best_wanted.prob_attempt),
                                    SortType::Price => perk_solver::result::format_price(best_wanted.price),
                                }
                            }
                            td { "{best_wanted.level}" }
                            td { MaterialName::vec_to_string(&best_wanted.mat_combination) }
                        }
                        if args.result_depth > 1 {
                            rsx!(
                                tr {
                                    th { colspan: 3, padding: "0", line_height: "1.5em", "Alts" }
                                }
                                for alt in perk_solver::result::find_best_alts(&result, args) {
                                    tr {
                                        td {
                                            match args.sort_type {
                                                SortType::Gizmo => perk_solver::result::format_float(alt.prob_gizmo),
                                                SortType::Attempt => perk_solver::result::format_float(alt.prob_attempt),
                                                SortType::Price => perk_solver::result::format_price(alt.price),
                                            }
                                        }
                                        td { "{alt.level}" }
                                        td { MaterialName::vec_to_string(&alt.mat_combination) }
                                    }
                                }
                            )
                        }
                    }
                }
            }
        ))
    } else {
        cx.render(rsx!(
            "No material combination found that can produce this gizmo."
        ))
    }
}

#[inline_props]
fn ProgressBar(
    cx: Scope,
    val: u64,
    max: u64,
    is_running: bool,
    ellapsed: time::Duration,
) -> Element {
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
                            input { r#type: "number", name: "invention level high", min: "1", max: "137", value: "137", margin_left: "3px" }
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
