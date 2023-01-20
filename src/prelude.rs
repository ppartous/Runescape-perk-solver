pub mod args;
pub use args::*;

pub mod perk_name;
pub use perk_name::*;

pub mod material_name;
pub use material_name::*;

pub mod data;
pub use data::*;

pub mod perk;
pub use perk::*;

pub mod gizmo;
pub use gizmo::*;

pub mod budget;
pub use budget::*;

use smallvec::{SmallVec, smallvec};
use std::sync::Arc;
use colored::Colorize;

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct PerkRankValuesProbabilityContainer {
    pub values: PerkRankValues,
    pub probability: f64,
}

pub type PRVPC = PerkRankValuesProbabilityContainer;

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct RankCombination {
    pub ranks: SmallVec<[PerkRankValues; 8]>,
    pub probability: f64
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PerkValues {
    pub name: PerkName,
    pub base: u16,
    pub rolls: SmallVec<[u8; 9]>,
    pub doubleslot: bool,
    pub ranks: SmallVec<[PerkRankValuesProbabilityContainer; 7]>,
    pub i_first: usize,
    pub i_last: usize,
}

impl Default for PerkValues {
    fn default() -> PerkValues {
        PerkValues {
            name: PerkName::Empty,
            base: 0,
            rolls: smallvec![],
            doubleslot: false,
            ranks: smallvec![],
            i_first: 0,
            i_last: 0
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SplitMaterials {
    pub conflict: Vec<MaterialName>,
    pub no_conflict: Vec<MaterialName>
}

impl std::fmt::Display for SplitMaterials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Materials".bright_green().underline())?;
        write!(f, " - Conflict: ")?;
        for (i, mat) in self.conflict.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", mat.to_string().cyan())?;
            } else {
                write!(f, "\n             {}", mat.to_string().cyan())?;
            }
        }
        write!(f, "\n - No conflict: ")?;
        for (i, mat) in self.no_conflict.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", mat.to_string().cyan())?;
            } else {
                write!(f, "\n                {}", mat.to_string().cyan())?;
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ResultLine {
    pub level: u8,
    pub prob_gizmo: f64,
    pub prob_attempt: f64,
    pub mat_combination: Arc<Vec<MaterialName>>
}

// ---------------------------------------------------------------------------------------------------------------------

// Keep separate to reduce memory usage
#[derive(Debug, Clone)]
pub struct ResultLineWithPrice {
    pub level: u8,
    pub prob_gizmo: f64,
    pub prob_attempt: f64,
    pub price: f64,
    pub mat_combination: Arc<Vec<MaterialName>>
}

impl Default for ResultLineWithPrice {
    fn default() -> Self {
        ResultLineWithPrice {
            level: 0,
            prob_gizmo: 0.0,
            prob_attempt: 0.0,
            price: f64::MAX,
            mat_combination: Arc::new(vec![])
        }
    }
}