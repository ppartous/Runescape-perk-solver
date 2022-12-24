use clap::{Parser, ValueEnum};
use derive_more::Display;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Perk to look for
    pub perk: String,

    /// Rank of the first perk
    #[arg(default_value_t = 1)]
    pub rank: u8,

    /// Second perk in the gizmo
    pub perk_two: Option<String>,

    /// Rank of the second perk
    #[arg(default_value_t = 1)]
    pub rank_two: u8,

    #[arg(value_enum, short('t'), long("type"))]
    pub gizmo_type: GizmoType,

    /// Invention level. Use two values separated by a comma to search in a range
    #[arg(short('l'), long("level"), required(true), use_value_delimiter = true, value_delimiter = ',')]
    pub invention_level: Vec<u32>,

    /// Is ancient gizmo
    #[arg(short, long)]
    pub ancient: bool,

    /// Use this if you don't care what the second perk is
    #[arg(short, long)]
    pub fuzzy: bool,

    /// Comma separated list of material values to exclude. Uses basic substring matching.
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    pub exclude: Vec<String>,

    /// Sort the result on probability per consumed gizmo, probability per attempt, or on estimated price.
    #[arg(value_enum, short, long, default_value_t = SortType::Gizmo)]
    pub sort_type: SortType
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