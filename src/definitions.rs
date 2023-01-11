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
use std::rc::Rc;

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
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

impl PerkValues {
    pub fn iter_ranks<'a>(self: &'a Self) -> impl Iterator<Item = &'a PerkRankValuesProbabilityContainer> {
        let i_first = self.i_first as usize;
        let i_last = self.i_last as usize;
        self.ranks.iter().skip(i_first).take(i_last - i_first + 1)
    }

    pub fn iter_ranks_no_zero<'a>(self: &'a Self) -> impl Iterator<Item = &'a PerkRankValuesProbabilityContainer> {
        let i_first = 1.max(self.i_first as usize);
        let i_last = self.i_last as usize;
        self.ranks.iter().skip(i_first).take(i_last - i_first + 1)
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SplitMaterials {
    pub conflict: Vec<MaterialName>,
    pub no_conflict: Vec<MaterialName>
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ResultLine {
    pub level: u16,
    pub prob_gizmo: f64,
    pub prob_attempt: f64,
    pub mat_combination: Rc<Vec<MaterialName>>
}