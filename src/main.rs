mod utils;
use perk_solver::{prelude::*, calc_gizmo_probabilities, perk_solver};
use std::str::FromStr;
use clap::Parser;

fn main() {
    let timer = howlong::HighResolutionTimer::new();
    let cli = Cli::parse();
    let data = Data::load();

    match cli.command {
        Commands::Gizmo {..} => {
            let args = Args::create(&cli);
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: args.perk, rank: args.rank },
                    Perk { name: args.perk_two, rank: args.rank_two }
                ),
                ..Default::default()
            };

            validate_input(&args, wanted_gizmo, &data);
            perk_solver(args, data, wanted_gizmo);
        },
        Commands::MaterialInput { mats } => {
            let mut materials = vec![];
            for mat_str in mats {
                let mat = MaterialName::from_str(&mat_str);
                match mat {
                    Ok(mat) => materials.push(mat),
                    Err(err) => utils::print_error(format!("{err} '{mat_str}'").as_str())
                }
            }
            let budget = Budget::create(cli.invention_level[0] as usize, cli.ancient);
            let gizmos = calc_gizmo_probabilities(&data, &budget, &materials, cli.gizmo_type, cli.ancient);

            for gizmo in gizmos {
                let prob_str = format!("{}", gizmo.probability);
                let zeros = prob_str.find(['1', '2', '3', '4', '5', '6', '7', '8', '9']).unwrap_or(2) - 2;
                println!("{:<20} {:<20}: {:.*}", format!("{} {},", gizmo.perks.0.name, gizmo.perks.0.rank),
                    format!("{} {},", gizmo.perks.1.name, gizmo.perks.1.rank), zeros + 4, gizmo.probability);
            }
        }
    }
    println!("{:?}", timer.elapsed());
}

fn validate_input(args: &Args, wanted_gizmo: Gizmo, data: &Data) {
    if data.perks[wanted_gizmo.perks.0.name].doubleslot && wanted_gizmo.perks.1.name != PerkName::Empty {
        utils::print_error(format!("Perk '{}' can't be combined with another perk as it uses both slots.", wanted_gizmo.perks.0.name).as_str())
    }
    if data.perks[wanted_gizmo.perks.1.name].doubleslot {
        utils::print_error(format!("Perk '{}' can't be combined with another perk as it uses both slots.", wanted_gizmo.perks.1.name).as_str())
    }

    if wanted_gizmo.perks.0.rank as usize >= data.perks[wanted_gizmo.perks.0.name].ranks.len() {
        utils::print_error(format!("Perk '{}' only goes up to rank {}.",
            &wanted_gizmo.perks.0.name,
            data.perks[wanted_gizmo.perks.0.name].ranks.len() - 1).as_str())
    }

    if wanted_gizmo.perks.1.name != PerkName::Empty && wanted_gizmo.perks.1.rank as usize >= data.perks[wanted_gizmo.perks.1.name].ranks.len() {
        utils::print_error(format!("Perk '{}' only goes up to rank {}.",
            &wanted_gizmo.perks.1.name,
            data.perks[wanted_gizmo.perks.1.name].ranks.len() - 1).as_str())
    }

    match args.invention_level {
        InventionLevel::Single(x) => {
            match x {
                1..=137 => (),
                _ => utils::print_error("Invention level must be between 1 and 137.")
            }
        },
        InventionLevel::Range(x, y) => {
            match (x, y) {
                (x, y) if x > y => utils::print_error("First value of the invention level range must be lower or equal to the second value."),
                (1..=137, 1..=137) => (),
                _ => utils::print_error("Invention level must be between 1 and 137.")
            }
        }
    }
}