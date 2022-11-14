use clap::Parser;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Perk to look for
    perk: String,

    /// Rank of the first perk
    #[arg(default_value_t = 1)]
    rank: u32,

    /// Second perk in the gizmo
    perk_two: Option<String>,

    /// Rank of the second perk
    #[arg(default_value_t = 1)]
    rank_two: u32,

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

    println!("{:?}", args);
}
