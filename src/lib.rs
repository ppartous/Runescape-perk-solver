pub mod definitions;
pub mod utils;
pub mod dice;
pub mod perk_values;
use definitions::*;
use itertools::Itertools;
use std::{cmp, fs};

pub fn load_data() -> Data {
    let data = fs::read_to_string("data.json").unwrap();
    serde_json::from_str(&data).unwrap()
}

#[allow(unused_variables, dead_code, unused_mut)]
pub fn perk_solver(args: &Args, data: &Data, wanted_gizmo: &WantedGizmo) {
    let materials = get_materials(&args, &data, &wanted_gizmo);
    let materials = split_materials(&args, &data, &wanted_gizmo, materials);
    let budgets = generate_budgets(&args);
    let slot_count = if args.ancient { 9 } else { 5 };

    let total_combination_count = calc_combination_count(materials.conflict.len(), materials.no_conflict.len(), args.ancient);
    let mut count = 0;
    let mut total_gizmos_generated = 0;

    println!("{:#?}", materials);
    println!("# combinations: {}", utils::format_int(total_combination_count as i64));

    for n_mats_used in 1..slot_count {
        // Order does no matter when none of the materials used have a cost conflict with the wanted perks
        for mat_combination in materials.no_conflict.iter().copied().combinations_with_replacement(n_mats_used) {
            calc_gizmo_probabilities(&data, &args, &budgets, &mat_combination, &wanted_gizmo);
        }
    }
}

fn get_materials(args: &Args, data: &Data, wanted_gizmo: &WantedGizmo) -> Vec<MaterialName> {
    let mut possible_materials = Vec::new();

    for (mat_name, mat_data) in data.comps.iter() {
        for comp_values in mat_data[args.gizmo_type].iter() {
            if (comp_values.perk == wanted_gizmo.0.perk || comp_values.perk == wanted_gizmo.1.perk)
            && (args.ancient || !mat_data.ancient_only) {
                possible_materials.push(*mat_name);
            }
        }
    }

    let possible_materials: Vec<_>= possible_materials.iter().unique().sorted().filter(|x| {
        !args.exclude.iter().any(|y| { x.to_string().to_lowercase().contains(&y.to_lowercase()) })
    }).copied().collect();

    if possible_materials.len() == 0 {
        utils::print_error("No materials found that can produce this perk.")
    }

    possible_materials
}

/// Splits material into two groups: conflict and non-conflict materials.
///
/// Conflict materials are materials that can generate perks with an equal cost value as one of the wanted perk ranks.
/// This matters as equal cost values can cause unstable sorting results so for these material combinations the order
/// of the materials if important whereas gizmos made entirely from non-conflict materials are position independent.
fn split_materials(args: &Args, data: &Data, wanted_gizmo: &WantedGizmo, mats: Vec<MaterialName>) -> SplitMaterials {
    let mut conflict = Vec::new();
    let mut no_conflict = Vec::new();

    let cost_p1 = data.perks[&wanted_gizmo.0.perk].ranks[wanted_gizmo.0.rank as usize].cost;
    let cost_p2 = if wanted_gizmo.1.perk != PerkName::Empty {
        data.perks[&wanted_gizmo.1.perk].ranks[wanted_gizmo.1.rank as usize].cost
    } else {
        0
    };

    for mat in mats.iter() {
        let mut is_conflict = false;
        'comp: for comp_values in data.comps[mat][args.gizmo_type].iter() {
            if comp_values.perk != wanted_gizmo.0.perk && comp_values.perk != wanted_gizmo.1.perk {
                for perk_rank in data.perks[&comp_values.perk].ranks.iter() {
                    if perk_rank.rank > 0 && (perk_rank.cost == cost_p1 || perk_rank.cost == cost_p2) {
                        conflict.push(*mat);
                        is_conflict = true;
                        break 'comp;
                    }
                }
            }
        }

        if !is_conflict {
            no_conflict.push(*mat);
        }
    }

    SplitMaterials { conflict, no_conflict }
}

/// Each budget is a cumulative probability distribution for the invention level related random rolls.
fn generate_budgets(args: &Args) -> Vec<Budget> {
    let rolls = if args.ancient { 6 } else { 5 };
    let (low, high) = if args.invention_level.len() == 1 {
        (args.invention_level[0], args.invention_level[0])
    } else {
        (args.invention_level[0], args.invention_level[1])
    };
    let mut budgets = Vec::new();

    for lvl in (low..=high).step_by(2) {
        let dist = dice::get_cumulative_distribution(lvl as usize / 2 + 20, rolls as usize);
        let max = dist.len() - 1;
        let budget = Budget {
            dist,
            level: lvl as u16,
            range: Range {
                min: lvl as u16,
                max: max as u16
            }
        };
        budgets.push(budget);
    }

    budgets
}

fn calc_combination_count(conflict_size: usize, no_conflict_size: usize, is_ancient: bool) -> usize {
    let slot_count = if is_ancient { 9 } else { 5 };
    let mut count = 0;

    fn fac(n: usize) -> usize {
        let mut r = 1;
        for i in 2..=n {
            r *= i;
        }
        r
    }

    for i in 1..=slot_count {
        count += dice::choose(no_conflict_size + i - 1, i) as usize;

        for j in 1..=cmp::min(i, conflict_size) {
            let mut x = 0;
            for k in 0..=cmp::min(i - j, no_conflict_size) {
                x += dice::choose(no_conflict_size, k) as usize * fac(j + k) * dice::choose(i - 1, i - j - k) as usize;
            }

            count += x * dice::choose(conflict_size, j) as usize;
        }
    }

    if no_conflict_size == 0 {
        count += slot_count;
    }

    count
}

#[allow(unused_variables, dead_code)]
fn calc_gizmo_probabilities(data: &Data, args: &Args, invent_budgets: &Vec<Budget>, input_materials: &Vec<MaterialName>, wanted_gizmo: &WantedGizmo) {
    todo!();
}