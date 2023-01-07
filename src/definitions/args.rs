use clap::{Parser, ValueEnum, Subcommand};
use derive_more::Display;

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
        #[arg(value_enum, short, long, default_value_t = SortType::Gizmo)]
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

#[derive(Debug)]
pub struct Args {
    pub invention_level: Vec<u32>,
    pub gizmo_type: GizmoType,
    pub ancient: bool,
    pub perk: String,
    pub rank: u8,
    pub perk_two: Option<String>,
    pub rank_two: u8,
    pub fuzzy: bool,
    pub exclude: Vec<String>,
    pub sort_type: SortType
}