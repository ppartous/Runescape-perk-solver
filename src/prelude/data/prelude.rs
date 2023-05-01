pub mod stack_vec;
use crate::{GizmoType, PerkName};
pub use stack_vec::*;
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct PerkRanksData {
    pub doubleslot: bool,
    pub ranks: StackVec<PerkRankValues, 7>, // 7 because no perk has more than 6 ranks + rank 0
}

impl Default for PerkRanksData {
    fn default() -> Self {
        PerkRanksData {
            doubleslot: false,
            ranks: StackVec::new(&[]),
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct ComponentValues {
    pub perk: PerkName,
    pub base: u8,
    pub roll: u8,
}

impl Default for ComponentValues {
    fn default() -> Self {
        ComponentValues {
            perk: PerkName::Empty,
            base: 0,
            roll: 0,
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct CompPerksPerGizmoType {
    pub ancient_only: bool,
    pub weapon: StackVec<ComponentValues, 7>,
    pub armour: StackVec<ComponentValues, 7>,
    pub tool: StackVec<ComponentValues, 7>,
}

impl Index<GizmoType> for CompPerksPerGizmoType {
    type Output = StackVec<ComponentValues, 7>;

    fn index(&self, index: GizmoType) -> &Self::Output {
        match index {
            GizmoType::Armour => &self.armour,
            GizmoType::Tool => &self.tool,
            GizmoType::Weapon => &self.weapon,
        }
    }
}

impl Default for CompPerksPerGizmoType {
    fn default() -> Self {
        CompPerksPerGizmoType {
            ancient_only: false,
            weapon: StackVec::new(&[]),
            armour: StackVec::new(&[]),
            tool: StackVec::new(&[]),
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerkRankValues {
    pub name: PerkName,
    pub rank: u8,
    pub cost: u16,
    pub threshold: u16,
    pub ancient_only: bool,
    pub doubleslot: bool,
}

impl Default for PerkRankValues {
    fn default() -> Self {
        PerkRankValues {
            ancient_only: false,
            cost: 0,
            doubleslot: false,
            name: PerkName::Empty,
            rank: 0,
            threshold: 0,
        }
    }
}

impl std::fmt::Display for PerkRankValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ name: {}, rank: {}, cost: {}, threshold: {}, ancient_only: {}, doubleslot: {} }}",
            self.name, self.rank, self.cost, self.threshold, self.ancient_only, self.doubleslot
        )
    }
}
