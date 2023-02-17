use crate::{component_prices::calc_gizmo_price, utils::print_warning, prelude::*};
use colored::*;
use std::{collections::HashMap, fs, sync::Arc};
use itertools::Itertools;
use tokio::sync::mpsc::Receiver;
use std::thread::JoinHandle;

pub fn result_handler(args: Arc<Args>, mut rx: Receiver<Arc<Vec<ResultLine>>>) -> JoinHandle<Vec<ResultLineWithPrice>> {
    let handler = std::thread::spawn(move || {
        let mut best_per_level = HashMap::new();
        match args.invention_level {
            InventionLevel::Single(x) => { best_per_level.insert(x, ResultLineWithPrice { ..Default::default() }); },
            InventionLevel::Range(x, y) => for lvl in (x..=y).step_by(2) {
                best_per_level.insert(lvl, ResultLineWithPrice { ..Default::default() });
            }
        }

        while let Some(lines) = rx.blocking_recv() {
            for line in lines.iter() {
                let price = calc_gizmo_price(line);
                let prev_best = best_per_level.get(&line.level).unwrap();

                let is_best = match args.sort_type {
                    SortType::Price => {
                        if price == prev_best.price {
                            line.mat_combination.len() < prev_best.mat_combination.len()
                        } else {
                            price < prev_best.price
                        }
                    },
                    SortType::Gizmo => {
                        if line.prob_gizmo == prev_best.prob_gizmo {
                            line.mat_combination.len() < prev_best.mat_combination.len()
                        } else {
                            line.prob_gizmo > prev_best.prob_gizmo
                        }
                    },
                    SortType::Attempt => {
                        if line.prob_attempt == prev_best.prob_attempt {
                            line.mat_combination.len() < prev_best.mat_combination.len()
                        } else {
                            line.prob_attempt > prev_best.prob_attempt
                        }
                    }
                };

                if is_best {
                    let line = ResultLineWithPrice {
                        level: line.level,
                        prob_attempt: line.prob_attempt,
                        prob_gizmo: line.prob_gizmo,
                        mat_combination: line.mat_combination.clone(),
                        price,
                    };
                    best_per_level.insert(line.level, line);
                }
            }
        }

        best_per_level.into_values().sorted_by(|x, y| x.level.cmp(&y.level))
            .filter(|x| x.prob_gizmo > 0.0).collect_vec()
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

pub fn print_result(best_per_level: &[ResultLineWithPrice], args: &Args) {
    let best_wanted_index = match args.sort_type {
        SortType::Price => best_per_level.iter().position_min_by(|a, b| a.price.partial_cmp(&b.price).unwrap()),
        SortType::Gizmo => best_per_level.iter().position_max_by(|a, b| a.prob_gizmo.partial_cmp(&b.prob_gizmo).unwrap()),
        SortType::Attempt => best_per_level.iter().position_max_by(|a, b| a.prob_attempt.partial_cmp(&b.prob_attempt).unwrap()),
    };

    if let Some(best_wanted_index) = best_wanted_index {
        println!("|-------|---------------------------|-----------|");
        println!("|       |      Probability (%)      |           |");
        println!("| Level |---------------------------|   Price   |");
        println!("|       |    Gizmo    |   Attempt   |           |");
        println!("|-------|---------------------------|-----------|");

        let best_wanted = &best_per_level[best_wanted_index];
        let best_gizmo = best_per_level.iter().map(|x| x.prob_gizmo).reduce(f64::max).unwrap();
        let best_attempt = best_per_level.iter().map(|x| x.prob_attempt).reduce(f64::max).unwrap();
        let best_price = best_per_level.iter().map(|x| x.price).reduce(f64::min).unwrap();

        for (i, line) in best_per_level.iter().enumerate() {
            let (r1, g1, b1) = get_color(line.prob_gizmo / best_gizmo);
            let (r2, g2, b2) = get_color(line.prob_attempt / best_attempt);
            let (r3, g3, b3) = get_color(best_price / line.price);

            print!("| {:>4}  |  {:>9}  |  {:>9}  | {:>9} |", line.level,
                format_float(line.prob_gizmo).truecolor(r1, g1, b1),
                format_float(line.prob_attempt).truecolor(r2, g2, b2),
                format_price(line.price).truecolor(r3, g3, b3));

            if i == best_wanted_index { println!(" <====") } else { println!() }
        }

        println!("|-------|---------------------------|-----------|\n");
        println!("Best combination at level {}:\n {}", best_wanted.level, MaterialName::vec_to_string_colored(best_wanted.mat_combination.as_ref()));
    } else {
        println!("No material combination found that can produce these perks.");
    }
}

pub fn write_best_mats_to_file(best_per_level: &[ResultLineWithPrice]) {
    let str = best_per_level.iter().map(|x| {
        format!("{}, {}", x.level, MaterialName::vec_to_string(x.mat_combination.as_ref()))
    }).join("\n");
    let res = fs::write("out.csv", str);
    if let Err(err) = res {
        print_warning(format!("Unable to write result to file: \"{}\"", err).as_str());
    }
}