use crate::{utils::*, MaterialName, PerkName};
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use derive_more::Display;
use itertools::Itertools;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(value_enum, short('t'), long("type"))]
    pub gizmo_type: GizmoType,

    /// Invention level. Use two values separated by a comma to search in a range
    #[arg(
        short('l'),
        long("level"),
        required(true),
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub invention_level: Vec<u8>,

    /// Is ancient gizmo
    #[arg(short, long)]
    pub ancient: bool,

    /// Show the gizmo probabilities related to a given set of materials
    #[command(subcommand)]
    pub command: Commands,
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Find the optimal material combination of a given gizmo
    Gizmo {
        /// Perk to look for
        perk: String,

        /// Rank of the first perk
        #[arg(default_value_t = 1)]
        rank: u8,

        /// Second perk in the gizmo. Use 'any' if you don't care what the second perk is. Leaving this field empty or
        /// using the string 'empty' means no second perk.
        perk_two: Option<String>,

        /// Rank of the second perk
        #[arg(default_value_t = 1)]
        rank_two: u8,

        /// Use this if you don't care what the second perk is. Is set automatically is second perk is 'any'
        #[arg(short, long)]
        fuzzy: bool,

        /// Comma separated list of material values to exclude. Uses basic substring matching
        #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
        exclude: Vec<String>,

        /// Sort the result on probability per consumed gizmo, probability per attempt, or on estimated price
        #[arg(value_enum, short, long, default_value_t = SortType::Price)]
        sort_type: SortType,

        /// Output file name. Set to false to disable output
        #[arg(long = "out-file", default_value_t = String::from("out.csv"))]
        out_file: String,

        /// Prices file name. Set to false to disable. When disabled prices are always loaded from the wiki
        #[arg(long = "price-file", default_value_t = String::from("prices.txt"))]
        price_file: String,

        /// Amount of alternative combinations to show
        #[arg(long = "alt-count", short = 'A', default_value_t = 0, value_parser = clap::value_parser!(u8).range(..=254))]
        alt_count: u8,

        /// Limit the number of threads used to 1 less than the amount available on the system
        #[arg(long = "limit-cpu", default_value_t = true)]
        limit_cpu: bool,
    },
    /// Show the gizmo probabilities for a given material combination
    MaterialInput {
        /// Comma separated list of materials. Shorter names are accepted (e.g. 'precise' instead of 'Precise components')
        #[arg(required(true), use_value_delimiter = true, value_delimiter = ',')]
        mats: Vec<String>,
    },
}

// ---------------------------------------------------------------------------------------------------------------------

/// Single letter aliases allowed
#[repr(C)]
#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum GizmoType {
    #[value(alias("w"))]
    Weapon,
    #[value(alias("a"))]
    Armour,
    #[value(alias("t"))]
    Tool,
}

// ---------------------------------------------------------------------------------------------------------------------

/// Single letter aliases allowed
#[repr(C)]
#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    #[value(alias("g"))]
    Gizmo,
    #[value(alias("a"))]
    Attempt,
    #[value(alias("p"))]
    Price,
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InventionLevel {
    Single(u8),
    Range(u8, u8),
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
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
    pub sort_type: SortType,
    pub out_file: Option<String>,
    pub price_file: Option<String>,
    pub result_depth: u8,
    pub limit_cpu: bool,
}

impl Args {
    pub fn create(cli: &Cli) -> Result<Args, String> {
        if let Commands::Gizmo {
            perk,
            rank,
            perk_two,
            rank_two,
            fuzzy,
            exclude,
            sort_type,
            out_file,
            price_file,
            alt_count,
            limit_cpu,
        } = &cli.command
        {
            let invention_level = match cli.invention_level.len() {
                0 => return Err("Missing invention level".to_string()),
                1 => InventionLevel::Single(cli.invention_level[0]),
                _ => InventionLevel::Range(cli.invention_level[0], cli.invention_level[1]),
            };

            let mut fuzzy = *fuzzy;

            let perk = match PerkName::from_str(perk) {
                Ok(perk) => perk,
                Err(_) => {
                    return Err(format!(
                        "Perk '{}' does not exist.",
                        perk.to_string().yellow()
                    ))
                }
            };

            let perk_two = if let Some(perk) = perk_two.as_ref() {
                if perk.to_lowercase() == "any" {
                    fuzzy = true;
                    PerkName::Empty
                } else {
                    match PerkName::from_str(perk) {
                        Ok(perk) => perk,
                        Err(_) => {
                            return Err(format!(
                                "Perk '{}' does not exist.",
                                perk.to_string().yellow()
                            ))
                        }
                    }
                }
            } else {
                PerkName::Empty
            };

            let rank_two = if perk_two == PerkName::Empty {
                0
            } else {
                *rank_two
            };

            let exclude = exclude.iter().filter_map(|x| {
                if x.is_empty() {
                    return None;
                }
                let mat = MaterialName::iter().filter(|mat| {
                    mat.to_string().to_lowercase().contains(x)
                }).collect_vec();
                if mat.is_empty() {
                    print_warning(format!("Ignoring exclude filter '{}' because it does not match with any material", x.yellow()).as_str());
                    None
                } else {
                    Some(mat)
                }
            }).flatten().collect_vec();

            let out_file = if out_file == "false" {
                None
            } else {
                Some(out_file.clone())
            };

            let price_file = if price_file == "false" {
                None
            } else {
                Some(price_file.clone())
            };

            Ok(Args {
                invention_level,
                gizmo_type: cli.gizmo_type,
                ancient: cli.ancient,
                perk,
                rank: *rank,
                perk_two,
                rank_two,
                fuzzy,
                sort_type: *sort_type,
                exclude,
                out_file,
                price_file,
                result_depth: *alt_count + 1,
                limit_cpu: *limit_cpu,
            })
        } else {
            Err("Bad command".to_string())
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
            sort_type: SortType::Price,
            out_file: Some(String::from("out.csv")),
            price_file: Some(String::from("prices.txt")),
            result_depth: 1,
            limit_cpu: false,
        }
    }
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Settings".underline().bright_green())?;
        match self.invention_level {
            InventionLevel::Single(x) => {
                writeln!(f, " - Invention level: {}", x.to_string().cyan())?
            }
            InventionLevel::Range(x, y) => writeln!(
                f,
                " - Invention level: {}-{}",
                x.to_string().cyan(),
                y.to_string().cyan()
            )?,
        }
        writeln!(f, " - Gizmo type: {}", self.gizmo_type.to_string().cyan())?;
        writeln!(f, " - Ancient gimzo: {}", self.ancient.to_string().cyan())?;
        if self.fuzzy {
            writeln!(
                f,
                " - Perk one: {} {}",
                self.perk.to_string().cyan(),
                self.rank.to_string().cyan()
            )?;
            writeln!(f, " - Perk two: {}", "Any".cyan())?;
        } else {
            writeln!(
                f,
                " - Perk one: {} {}",
                self.perk.to_string().cyan(),
                self.rank.to_string().cyan()
            )?;
            if self.perk_two != PerkName::Empty {
                writeln!(
                    f,
                    " - Perk two: {} {}",
                    self.perk_two.to_string().cyan(),
                    self.rank_two.to_string().cyan()
                )?;
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
        if !self.exclude.is_empty() {
            write!(
                f,
                "\n - Excluded materials: {}",
                self.exclude.iter().map(|x| x.to_string().cyan()).join(", ")
            )?;
        }
        Ok(())
    }
}
