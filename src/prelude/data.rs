use serde::Deserialize;
use std::{collections::HashMap, ops::Index};
use derive_more::Display;
use crate::{PerkName, MaterialName, GizmoType};

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Data {
    pub comps: HashMap<MaterialName, CompPerksPerGizmoType>,
    pub perks: HashMap<PerkName, PerkRanksData>
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct PerkRanksData {
    pub doubleslot: bool,
    pub ranks: Vec<PerkRankValues>,
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ComponentValues {
    pub base: u16,
    pub perk: PerkName,
    pub roll: u16,
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CompPerksPerGizmoType {
    pub ancient_only: bool,
    pub armour: Vec<ComponentValues>,
    pub tool: Vec<ComponentValues>,
    pub weapon: Vec<ComponentValues>,
}

impl Index<GizmoType> for CompPerksPerGizmoType {
    type Output = Vec<ComponentValues>;

    fn index(&self, index: GizmoType) -> &Self::Output {
        match index {
                GizmoType::Armour => &self.armour,
                GizmoType::Tool => &self.tool,
                GizmoType::Weapon => &self.weapon
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Display)]
#[display(fmt = "{{\n\tname = {},\n\trank = {},\n\tdoubleslot = {},\n\tancient_only = {},\n\tcost = {},\n\tthreshold = {}\n}}", name, rank, doubleslot, ancient_only, cost, threshold)]
pub struct PerkRankValues {
    pub ancient_only: bool,
    pub cost: u16,
    pub doubleslot: bool,
    pub name: PerkName,
    pub rank: u8,
    pub threshold: u16,
}

impl Default for PerkRankValues {
    fn default() -> Self {
        PerkRankValues {
            ancient_only: false,
            cost: 0,
            doubleslot: false,
            name: PerkName::Empty,
            rank: 0,
            threshold: 0
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

