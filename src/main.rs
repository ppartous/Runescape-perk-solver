mod utils;
use perk_solver::{prelude::*, calc_gizmo_probabilities, perk_solver};
use std::str::FromStr;
use clap::Parser;

fn main() {
    #[cfg(feature="precise-time")]
    {
        let timer = howlong::HighResolutionTimer::new();
    }
    let cli = Cli::parse();
    let data = Data::load();

    match cli.command {
        Commands::Gizmo {..} => {
            let args = Args::create(&cli).unwrap_or_else(|err| utils::print_error(err.as_str()));
            perk_solver(args, data);
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

    #[cfg(feature="precise-time")]
    {
        println!("{:?}", timer.elapsed());
    }
}