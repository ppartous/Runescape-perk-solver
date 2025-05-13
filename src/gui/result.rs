use crate::wiki::{make_wiki_calc_link, WikiImage};
use dioxus::prelude::*;
use itertools::Itertools;
use perk_solver::prelude::*;

pub fn FullResultTable<'a>(cx: Scope<'a>, result: &Vec<Vec<ResultLine>>) -> Element<'a> {
    cx.render(rsx!(
        table {
            class: "wikitable",
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
                    td {
                        rowspan: "{lines.len()}",
                        match lines[0].level - (lines[0].level % 2) {
                            0 => String::from("1"),
                            x => format!("{x}-{}", x + 1)
                        }
                    }
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

pub fn ResultTable<'a>(cx: Scope<'a>, result: &Vec<Vec<ResultLine>>, args: &Args) -> Element<'a> {
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
                    class: "wikitable",
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
                        class: "wikitable result-alts align-left-3 align-center-4",
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
                            th { "Link" }
                        }
                        tr {
                            th { colspan: 4, padding: "0", line_height: "1.5em", "Best" }
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
                            td {
                                MatCombinationList(cx, &best_wanted.mat_combination)
                            }
                            td {
                                a {
                                    href: "{make_wiki_calc_link(&best_wanted.mat_combination, args.ancient, args.gizmo_type, best_wanted.level).as_str()}",
                                    class: "calc-link",
                                    title: "Open on the wiki"
                                }
                            }
                        }
                        if args.result_depth > 1 {
                            rsx!(
                                tr {
                                    th { colspan: 4, padding: "0", line_height: "1.5em", "Alts" }
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
                                        td {
                                            MatCombinationList(cx, &alt.mat_combination)
                                        }
                                        td {
                                            a {
                                                href: "{make_wiki_calc_link(&alt.mat_combination, args.ancient, args.gizmo_type, alt.level).as_str()}",
                                                class: "calc-link",
                                                title: "Open on the wiki"
                                            }
                                        }
                                    }
                                }
                            )
                        }
                    }
                    p {
                        div {
                            "The materials are in the order in which they fill the gizmo when clicked upon."
                        }
                        div {
                            class: "mat-order-image",
                            table {
                                tr {
                                    td { "6" }
                                    td { "2" }
                                    td { "7" }
                                }
                                tr {
                                    td { "3" }
                                    td { "1" }
                                    td { "4" }
                                }
                                tr {
                                    td { "8" }
                                    td { "5" }
                                    td { "9" }
                                }
                            }
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

fn MatCombinationList<'a>(cx: Scope<'a>, mat_combinaton: &[MaterialName]) -> Element<'a> {
    let counts = mat_combinaton.iter().counts();
    cx.render(rsx!(
        for (i, mat) in mat_combinaton.iter().unique().enumerate() {
            if i > 0 {
                rsx!(", ")
            }
            "{*counts.get(mat).unwrap()} × "
            if COMMON_MATERIALS.contains(mat) {
                rsx!{"5 "}
            }
            WikiImage(cx, mat.to_str())
            "{mat}"
        }
    ))
}

fn get_color(val: f64) -> (u8, u8, u8) {
    let val = val.powi(10);
    (
        ((1.0 - val.clamp(0.5, 1.0)) * 2.0 * 190.0) as u8,
        (val.clamp(0.0, 0.5) * 2.0 * 190.0) as u8,
        0,
    )
}
