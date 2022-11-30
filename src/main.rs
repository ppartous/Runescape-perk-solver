use perk_solver::{definitions::*, utils, load_data};
use std::{str::FromStr};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let data = load_data();
    let wanted_gizmo = process_wanted_gizmo(&args);

    validate_input(&args, &wanted_gizmo, &data);

    println!("{:#?}", args);
    println!("{:#?}", wanted_gizmo);

    perk_solver::perk_solver(&args, &data, &wanted_gizmo);
}

fn process_wanted_gizmo(args: &Args) -> Gizmo {
    let perk_one = PerkName::from_str(&args.perk).unwrap_or_else(|_| {
        utils::print_error(format!("Perk '{}' does not exist.", &args.perk).as_str())
    });

    let perk_two = if let Some(perk) = &args.perk_two {
        PerkName::from_str(&perk).unwrap_or_else(|_| {
            utils::print_error(format!("Perk '{}' does not exist.", &perk).as_str())
        })
    } else {
        PerkName::Empty
    };

    Gizmo {
        perks: (
            Perk {
                perk: perk_one,
                rank: args.rank,
            },
            Perk {
                perk: perk_two,
                rank: if let Some(_) = &args.perk_two { args.rank_two } else { 0 },
            }
        ),
        ..Default::default()
    }
}

fn validate_input(args: &Args, wanted_gizmo: &Gizmo, data: &Data) {
    if data.perks[&wanted_gizmo.perks.0.perk].doubleslot && wanted_gizmo.perks.1.perk != PerkName::Empty {
        utils::print_error(format!("Perk '{}' can't be combined with another perk as it uses both slots.", wanted_gizmo.perks.0.perk).as_str())
    }
    if data.perks[&wanted_gizmo.perks.1.perk].doubleslot {
        utils::print_error(format!("Perk '{}' can't be combined with another perk as it uses both slots.", wanted_gizmo.perks.1.perk).as_str())
    }

    if wanted_gizmo.perks.0.rank as usize >= data.perks[&wanted_gizmo.perks.0.perk].ranks.len() {
        utils::print_error(format!("Perk '{}' only goes up to rank {}.",
            &wanted_gizmo.perks.0.perk,
            data.perks[&wanted_gizmo.perks.0.perk].ranks.len() - 1).as_str())
    }

    if &wanted_gizmo.perks.1.perk != &PerkName::Empty && wanted_gizmo.perks.1.rank as usize >= data.perks[&wanted_gizmo.perks.1.perk].ranks.len() {
        utils::print_error(format!("Perk '{}' only goes up to rank {}.",
            &wanted_gizmo.perks.1.perk,
            data.perks[&wanted_gizmo.perks.1.perk].ranks.len() - 1).as_str())
    }

    match &args.invention_level.len() {
        1 => {
            match &args.invention_level[0] {
                1..=137 => (),
                _ => utils::print_error("Invention level must be between 1 and 137.")
            }
        },
        _ => {
            match &args.invention_level[0..=1] {
                [x, y] if x > y => utils::print_error("First value of the invention level range must be lower or equal to the second value."),
                [1..=137, 1..=137] => (),
                _ => utils::print_error("Invention level must be between 1 and 137.")
            }
        }
    }
}