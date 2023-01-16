use std::str::FromStr;
use clap::{Parser, ValueEnum, Subcommand};
use derive_more::Display;
use colored::*;
use itertools::Itertools;
use crate::{PerkName, MaterialName, utils::*};


#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(value_enum, short('t'), long("type"))]
    pub gizmo_type: GizmoType,

    /// Invention level. Use two values separated by a comma to search in a range
    #[arg(short('l'), long("level"), required(true), use_value_delimiter = true, value_delimiter = ',')]
    pub invention_level: Vec<u32>,

    /// Is ancient gizmo
    #[arg(short, long)]
    pub ancient: bool,

    /// Show the gizmo probabilities related to a given set of materials
    #[command(subcommand)]
    pub command: Commands
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum Commands {
    Gizmo {
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

        /// Use this if you don't care what the second perk is
        #[arg(short, long)]
        fuzzy: bool,

        /// Comma separated list of material values to exclude. Uses basic substring matching.
        #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
        exclude: Vec<String>,

        /// Sort the result on probability per consumed gizmo, probability per attempt, or on estimated price.
        #[arg(value_enum, short, long, default_value_t = SortType::Price)]
        sort_type: SortType,
    },
    MaterialInput {
        #[arg(required(true), use_value_delimiter = true, value_delimiter = ',')]
        mats: Vec<String>
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum GizmoType {
    Weapon,
    Armour,
    Tool
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    Gizmo,
    Attempt,
    Price
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InventionLevel {
    Single (u32),
    Range (u32, u32)
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Args {
    pub invention_level: InventionLevel,
    pub gizmo_type: GizmoType,
    pub ancient: bool,
    pub perk: PerkName,
    pub rank: u8,
    pub perk_two: PerkName,
    pub rank_two: u8,
    pub fuzzy: bool,
    pub exclude: Vec<MaterialName>,
    pub sort_type: SortType
}

impl Args {
    pub fn create(cli: &Cli) -> Args {
        if let Commands::Gizmo { perk, rank, perk_two, rank_two, fuzzy, exclude, sort_type } = &cli.command {
            let invention_level = match cli.invention_level.len() {
                0 => print_error("Missing invention level"),
                1 => InventionLevel::Single(cli.invention_level[0]),
                _ => InventionLevel::Range(cli.invention_level[0], cli.invention_level[1])
            };

            let perk = PerkName::from_str(perk).unwrap_or_else(|_| {
                print_error(format!("Perk '{}' does not exist.", &perk).as_str())
            });

            let perk_two = if let Some(perk) = perk_two.as_ref() {
                PerkName::from_str(perk).unwrap_or_else(|_| {
                    print_error(format!("Perk '{}' does not exist.", &perk).as_str())
                })
            } else {
                PerkName::Empty
            };

            let rank_two = if perk_two == PerkName::Empty { 0 } else { *rank_two };

            let exclude = exclude.iter().filter_map(|x| {
                let mat = MaterialName::iter().find(|mat| {
                    mat.to_string().to_lowercase().contains(x)
                });
                if mat.is_none() { print_warning(format!("Ignoring exclude filter '{}' because it does not match with any material", x).as_str()) }
                mat
            }).collect_vec();

            Args {
                invention_level,
                gizmo_type: cli.gizmo_type,
                ancient: cli.ancient,
                perk,
                rank: *rank,
                perk_two,
                rank_two,
                fuzzy: *fuzzy,
                sort_type: *sort_type,
                exclude
            }
        } else {
            print_error("Bad command");
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Args {
            invention_level: InventionLevel::Single(1),
            gizmo_type: GizmoType::Weapon,
            ancient: false,
            perk: PerkName::Empty,
            rank: 0,
            perk_two: PerkName::Empty,
            rank_two: 0,
            fuzzy: false,
            exclude: vec![],
            sort_type: SortType::Price
        }
    }
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Settings".underline().bright_green())?;
        match self.invention_level {
            InventionLevel::Single(x) => writeln!(f, " - Invention level: {}", x.to_string().cyan())?,
            InventionLevel::Range(x, y) => writeln!(f, " - Invention level: {}-{}", x.to_string().cyan(), y.to_string().cyan())?
        }
        writeln!(f, " - Gizmo type: {}", self.gizmo_type.to_string().cyan())?;
        writeln!(f, " - Ancient gimzo: {}", self.ancient.to_string().cyan())?;
        if self.fuzzy {
            writeln!(f, " - Perk one: {} {}", self.perk.to_string().cyan(), self.rank.to_string().cyan())?;
            writeln!(f, " - Perk two: {}", "Any".cyan())?;
        } else {
            writeln!(f, " - Perk one: {} {}", self.perk.to_string().cyan(), self.rank.to_string().cyan())?;
            if self.perk_two != PerkName::Empty {
                writeln!(f, " - Perk two: {} {}", self.perk_two.to_string().cyan(), self.rank_two.to_string().cyan())?;
            } else {
                writeln!(f, " - Perk two: {}", "Empty".cyan())?;
            }
        }
        let sort_type = match self.sort_type {
            SortType::Attempt => "probability per attemp",
            SortType::Gizmo => "probability per consumed gizmo",
            SortType::Price => "estimated price",
        };
        write!(f, " - Sort on {}", sort_type.cyan())?;
        if !self.exclude.is_empty()  {
            write!(f, "\n - Excluded materials: {}", self.exclude.iter().map(|x| x.to_string().cyan()).join(", "))?;
        }
        Ok(())
    }
}