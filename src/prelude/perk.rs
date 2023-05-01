use crate::{PerkName, PerkRankValues};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Perk {
    pub name: PerkName,
    pub rank: u8,
}

impl Default for Perk {
    fn default() -> Self {
        Perk {
            name: PerkName::Empty,
            rank: 0,
        }
    }
}

impl PartialEq<PerkRankValues> for Perk {
    fn eq(&self, other: &PerkRankValues) -> bool {
        (self.name == other.name) & (self.rank == other.rank)
    }
}

impl PartialEq<Perk> for PerkRankValues {
    fn eq(&self, other: &Perk) -> bool {
        other == self
    }
}

impl From<&PerkRankValues> for Perk {
    fn from(x: &PerkRankValues) -> Self {
        Perk {
            name: x.name,
            rank: x.rank,
        }
    }
}

impl Perk {
    pub fn is_empty(&self) -> bool {
        self.name == PerkName::Empty
    }
}
