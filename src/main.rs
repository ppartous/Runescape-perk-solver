use clap::Parser;
use std::{str::FromStr, fs};
mod utils;
use utils::{PerkName, WantedGizmo, Perk};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Perk to look for
    perk: String,

    /// Rank of the first perk
    #[arg(default_value_t = 1)]
    rank: u8,

    /// Second perk in the gizmo
    perk_two: Option<String>,

    /// Rank of the second perk
    #[arg(default_value_t = 1)]
    rank_two: u8,

    #[arg(value_enum, short('t'), long("type"))]
    gizmo_type: utils::GizmoType,

    /// Invention level. Use two values separated by a comma to search in a range
    #[arg(short('l'), long("level"), required(true), use_value_delimiter = true, value_delimiter = ',')]
    invention_level: Vec<u32>,

    /// Is ancient gizmo
    #[arg(short, long)]
    ancient: bool,

    /// Use this if you don't care what the second perk is
    #[arg(short, long)]
    fuzzy: bool,

    /// Comma separated list of material values to exclude. Uses basic substring matching.
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    exclude: Vec<String>,

    /// Sort the result on probability per consumed gizmo, probability per attempt, or on estimated price.
    #[arg(value_enum, short, long, default_value_t = utils::SortType::Price)]
    sort_type: utils::SortType
}

fn main() {
    let args = Args::parse();
    let data = fs::read_to_string("data.json").unwrap();
    let data: utils::Data = serde_json::from_str(&data).unwrap();
    let wanted_gizmo = process_wanted_gizmo(&args, &data);

    validate_input(&args, &wanted_gizmo, &data);

    println!("{:#?}", args);
    println!("{:#?}", wanted_gizmo);
}

fn process_wanted_gizmo(args: &Args, data: &utils::Data) -> WantedGizmo {
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

    WantedGizmo(
        Perk {
            perk: perk_one,
            rank: args.rank,
            doubleslot: data.perks[&perk_one].doubleslot
        },
        Perk {
            perk: perk_two,
            rank: if let Some(_) = &args.perk_two { args.rank_two } else { 0 },
            doubleslot: if perk_two != PerkName::Empty { data.perks[&perk_two].doubleslot } else { false }
        }
    )
}

fn validate_input(args: &Args, wanted_gizmo: &WantedGizmo, data: &utils::Data) {
    if wanted_gizmo.0.doubleslot && wanted_gizmo.1.perk != PerkName::Empty {
        utils::print_error(format!("Perk '{}' can't be combined with another perk as it uses both slots.", wanted_gizmo.0.perk).as_str())
    }
    if wanted_gizmo.1.doubleslot {
        utils::print_error(format!("Perk '{}' can't be combined with another perk as it uses both slots.", wanted_gizmo.1.perk).as_str())
    }

    if wanted_gizmo.0.rank as usize >= data.perks[&wanted_gizmo.0.perk].ranks.len() {
        utils::print_error(format!("Perk '{}' only goes up to rank {}.",
            &wanted_gizmo.0.perk,
            data.perks[&wanted_gizmo.0.perk].ranks.len() - 1).as_str())
    }

    if &wanted_gizmo.1.perk != &PerkName::Empty && wanted_gizmo.1.rank as usize >= data.perks[&wanted_gizmo.1.perk].ranks.len() {
        utils::print_error(format!("Perk '{}' only goes up to rank {}.",
            &wanted_gizmo.1.perk,
            data.perks[&wanted_gizmo.1.perk].ranks.len() - 1).as_str())
    }

    match &args.invention_level.len() {
        1 => {
            match &args.invention_level[0] {
                0 => utils::print_error("Invention level must be greater than 0."),
                n if *n > 137 => utils::print_error("Invention level must be between 1 and 137."),
                _ => ()
            }
        },
        _ => {
            match &args.invention_level[0..2] {
                [0, _] | [_, 0]  => utils::print_error("Invention level must be greater than 0."),
                [n, _] | [_, n] if *n > 137 => utils::print_error("Invention level must be between 1 and 137."),
                [x, y] if x > y => utils::print_error("First value of the invention level range must be lower or equal to the second value."),
                _ => ()
            }
        }
    }
}