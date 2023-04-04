use crate::{prelude::*, utils::print_warning};
use colored::*;
use itertools::Itertools;
use std::{
    collections::HashMap,
    fs,
    sync::{mpsc::Receiver, Arc},
    thread::JoinHandle,
};

pub fn result_handler(
    args: Arc<Args>,
    rx: Receiver<Vec<ResultLine>>,
) -> JoinHandle<Vec<Vec<ResultLine>>> {
    let handler = std::thread::spawn(move || {
        let mut best_per_level = HashMap::new();
        match args.invention_level {
            InventionLevel::Single(x) => {
                best_per_level.insert(
                    x,
                    vec![
                        ResultLine {
                            ..Default::default()
                        };
                        args.result_depth as usize
                    ],
                );
            }
            InventionLevel::Range(x, y) => {
                for lvl in (x..=y).step_by(2) {
                    best_per_level.insert(
                        lvl,
                        vec![
                            ResultLine {
                                ..Default::default()
                            };
                            args.result_depth as usize
                        ],
                    );
                }
            }
        }

        while let Ok(lines) = rx.recv() {
            for line in lines.into_iter() {
                let current_bests = best_per_level.get_mut(&line.level).unwrap();

                if let Some((i, x)) = current_bests
                    .iter()
                    .find_position(|x| line.is_better(x, args.sort_type))
                {
                    if !line.eq(x, args.sort_type) {
                        current_bests.pop();
                        current_bests.insert(i, line);
                    }
                }
            }
        }

        best_per_level
            .into_values()
            .sorted_by(|x, y| x[0].level.cmp(&y[0].level))
            .filter(|x| x[0].prob_gizmo > 0.0)
            .collect_vec()
    });

    handler
}

fn format_float(num: f64) -> String {
    let num = num.min(1.0) * 100.0;
    if num > 1e-2 {
        format!("{:.5}", num)
    } else if num > 1e-9 {
        format!("{:.4e}", num)
    } else if num > 1e-99 {
        format!("{:.3e}", num)
    } else {
        format!("{:.2e}", num)
    }
}

fn format_price(num: f64) -> String {
    if num < 1e4 {
        format!("{}", num as usize)
    } else if num < 1e7 {
        format!("{} k", (num / 1e3) as usize)
    } else if num < 1e10 {
        format!("{:.1} M", num / 1e6)
    } else if num < 1e13 {
        format!("{:.1} B", num / 1e9)
    } else {
        format!("{:.2e}", num)
    }
}

fn get_color(ratio: f64) -> (u8, u8, u8) {
    if ratio > 0.98 {
        (44, 186, 0) // Green
    } else if ratio > 0.95 {
        (149, 229, 0) // Light green
    } else if ratio > 0.90 {
        (255, 244, 0) // Yellow
    } else if ratio > 0.50 {
        (255, 167, 0) // Orange
    } else if ratio > 0.10 {
        (219, 108, 108) // Red
    } else {
        (255, 0, 0) // Strong red
    }
}

pub fn print_result(best_per_level: &Vec<Vec<ResultLine>>, args: &Args) {
    if let Some((best_gizmo_index, best_attempt_index, best_price_index)) =
        get_best_of_each(best_per_level)
    {
        let best_wanted_index = match args.sort_type {
            SortType::Gizmo => best_gizmo_index,
            SortType::Attempt => best_attempt_index,
            SortType::Price => best_price_index,
        };

        let best_wanted = &best_per_level[best_wanted_index][0];
        let best_gizmo_prob = best_per_level[best_gizmo_index][0].prob_gizmo;
        let best_attempt_prob = best_per_level[best_attempt_index][0].prob_attempt;
        let best_price = best_per_level[best_price_index][0].price;

        println!("|-------|---------------------------|-----------|");
        println!("|       |      Probability (%)      |           |");
        println!("| Level |---------------------------|   Price   |");
        println!("|       |    Gizmo    |   Attempt   |           |");
        println!("|-------|---------------------------|-----------|");

        for (i, line) in best_per_level.iter().enumerate() {
            let (r1, g1, b1) = get_color(line[0].prob_gizmo / best_gizmo_prob);
            let (r2, g2, b2) = get_color(line[0].prob_attempt / best_attempt_prob);
            let (r3, g3, b3) = get_color(best_price / line[0].price);

            print!(
                "| {:>4}  |  {:>9}  |  {:>9}  | {:>9} |",
                line[0].level,
                format_float(line[0].prob_gizmo).truecolor(r1, g1, b1),
                format_float(line[0].prob_attempt).truecolor(r2, g2, b2),
                format_price(line[0].price).truecolor(r3, g3, b3)
            );

            if i == best_wanted_index {
                println!(" <====")
            } else {
                println!()
            }
        }

        println!("|-------|---------------------------|-----------|\n");

        let val = match args.sort_type {
            SortType::Price => format_price(best_per_level[best_wanted_index][0].price),
            SortType::Gizmo => format!(
                "{}%",
                format_float(best_per_level[best_wanted_index][0].prob_gizmo)
            ),
            SortType::Attempt => format!(
                "{}%",
                format_float(best_per_level[best_wanted_index][0].prob_attempt)
            ),
        };
        println!(
            "Best combination at level {}:\n {:<8}: {}",
            best_wanted.level,
            val,
            MaterialName::vec_to_string(best_wanted.mat_combination.as_ref())
        );

        if args.result_depth > 1 {
            println!("\nAlts:");
            for alt in find_best_alts(best_per_level, args) {
                let val = match args.sort_type {
                    SortType::Price => format_price(alt.price),
                    SortType::Gizmo => format!("{}%", format_float(alt.prob_gizmo)),
                    SortType::Attempt => format!("{}%", format_float(alt.prob_attempt)),
                };
                println!(
                    " {:<8} @lvl {}: {}",
                    val,
                    alt.level,
                    MaterialName::vec_to_string(alt.mat_combination.as_ref())
                );
            }
        }

        #[cfg(feature = "wiki-template")]
        {
            let counts = best_wanted.mat_combination.iter().counts();
            let mut mats = vec![];
            for mat in best_wanted.mat_combination.iter().unique() {
                let s = mat
                    .to_string()
                    .replace(" parts", "")
                    .replace(" components", "");
                for _ in 0..counts[mat] {
                    mats.push(s.clone());
                }
            }
            if mats.len() >= 3 {
                mats.swap(0, 2);
                mats.swap(0, 1);
            }
            let mats = mats.join("|");
            let gizmo_type = args.gizmo_type.to_string().to_lowercase();
            let gizmo_image = match args.ancient {
                true => format!("Ancient {gizmo_type}"),
                false => {
                    let (x, y) = gizmo_type.split_at(1);
                    format!("{}{y}", x.to_uppercase())
                }
            };
            let (level, potion) = match best_wanted.level {
                ..=120 => ((best_wanted.level / 2) * 2, "none"),
                121..=123 => (120, "normal"),
                124..=125 => (120, "super"),
                126.. => (120, "extreme"),
            };
            println!("[[File:{gizmo_image} gizmo.png|link=]] {{{{Perk calclink|{mats}|gizmo={gizmo_type}|level={level}|potion={potion}|text='''Probability: {:.2}%'''}}}}", best_wanted.prob_gizmo * 100.0);
            println!("{{{{Perk calclink|{mats}|gizmo={gizmo_type}|level={level}|potion={potion}|text={} {}, {} {}}}}} ({:.2}%)",
                args.perk, args.rank, args.perk_two, args.rank_two, best_wanted.prob_gizmo * 100.0);
        }
    } else {
        println!("No material combination found that can produce this gizmo.");
    }
}

pub fn get_best_of_each(best_per_level: &Vec<Vec<ResultLine>>) -> Option<(usize, usize, usize)> {
    let best_gizmo = best_per_level
        .iter()
        .position_max_by(|x, y| x[0].prob_gizmo.partial_cmp(&y[0].prob_gizmo).unwrap());
    let best_attempt = best_per_level
        .iter()
        .position_max_by(|x, y| x[0].prob_attempt.partial_cmp(&y[0].prob_attempt).unwrap());
    let best_price = best_per_level
        .iter()
        .position_min_by(|x, y| x[0].price.partial_cmp(&y[0].price).unwrap());

    if best_gizmo.is_some() {
        Some((
            best_gizmo.unwrap(),
            best_attempt.unwrap(),
            best_price.unwrap(),
        ))
    } else {
        None
    }
}

pub fn find_best_alts<'a>(
    best_per_level: &'a Vec<Vec<ResultLine>>,
    args: &Args,
) -> Vec<&'a ResultLine> {
    best_per_level
        .iter()
        .flatten()
        .filter(|x| x.prob_gizmo > 0.0)
        .sorted_by(|x, y| {
            if x.eq(y, args.sort_type) {
                x.level.cmp(&y.level)
            } else if x.is_better(y, args.sort_type) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unique_by(|x| gizmo_combination_sort(&x.mat_combination))
        .skip(1)
        .take(args.result_depth as usize - 1)
        .collect()
}

pub fn write_best_mats_to_file(best_per_level: &Vec<Vec<ResultLine>>, args: &Args) {
    if args.out_file == "false" {
        return;
    }

    colored::control::set_override(false);
    let str = best_per_level
        .iter()
        .cloned()
        .flatten()
        .filter(|x| x.prob_gizmo > 0.0)
        .map(|x| {
            format!(
                "{}, {:.3e}, {:.3e}, {:.3e}, {}",
                x.level,
                x.prob_gizmo * 100.0,
                x.prob_attempt * 100.0,
                x.price,
                MaterialName::vec_to_string(x.mat_combination.as_ref())
            )
        })
        .join("\n");
    let str = format!(
        "Level, Prob gizmo (%), Prob attemp (%), Price, Materials\n{}",
        str
    );
    let res = fs::write(&args.out_file, str);
    colored::control::unset_override();
    if let Err(err) = res {
        print_warning(format!("Unable to write result to file: {}", err).as_str());
    }
}

pub fn gizmo_combination_sort(v: &[MaterialName]) -> Vec<MaterialName> {
    let counts = v.iter().counts();
    v.iter()
        .copied()
        .unique()
        .map(|x| {
            let count = *counts.get(&x).unwrap();
            vec![x; count]
        })
        .flatten()
        .collect_vec()
}
