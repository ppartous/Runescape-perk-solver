mod utils;
use clap::Parser;
use perk_solver::{calc_gizmo_probabilities, perk_solver, prelude::*};
use std::str::FromStr;

fn main() {
    #[cfg(feature = "precise-time")]
    let timer = howlong::HighResolutionTimer::new();

    let cli = Cli::parse();

    match cli.command {
        Commands::Gizmo { .. } => {
            let args = Args::create(&cli).unwrap_or_else(|err| utils::print_error(err.as_str()));
            perk_solver(args);
        }
        Commands::MaterialInput { mats } => {
            let data = Data::load();
            let mut materials = vec![];
            for mat_str in mats {
                let mat = MaterialName::from_str(&mat_str);
                match mat {
                    Ok(mat) => materials.push(mat),
                    Err(err) => utils::print_error(format!("{err} '{mat_str}'").as_str()),
                }
            }
            if (!cli.ancient && materials.len() > 5) || materials.len() > 9 {
                utils::print_error("Too many materials")
            }
            if cli.invention_level[0] == 0 || cli.invention_level[0] > 137 {
                utils::print_error("Invalid invention level")
            }
            let budget = Budget::create(cli.invention_level[0] as usize, cli.ancient);
            let gizmos =
                calc_gizmo_probabilities(&data, &budget, &materials, cli.gizmo_type, cli.ancient);

            for gizmo in gizmos {
                let prob_str = format!("{}", gizmo.probability);
                let zeros = prob_str
                    .find(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
                    .unwrap_or(2)
                    - 2;
                println!(
                    "{:<20} {:<20}: {:.*}",
                    perk_to_string(&data, gizmo.perks.0.name, gizmo.perks.0.rank),
                    perk_to_string(&data, gizmo.perks.1.name, gizmo.perks.1.rank),
                    zeros + 4,
                    gizmo.probability
                );
            }
        }
    }

    #[cfg(feature = "precise-time")]
    println!("\n{:?}", timer.elapsed());
}

fn perk_to_string(data: &Data, perk: PerkName, rank: u8) -> String {
    if data.perks[perk].ranks.len() <= 2 {
        perk.to_string()
    } else {
        format!("{} {}", perk, rank)
    }
}
