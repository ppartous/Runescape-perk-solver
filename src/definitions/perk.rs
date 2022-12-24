use crate::{PerkName, PerkRankValues};

#[derive(Debug, PartialEq)]
pub struct Perk {
    pub perk: PerkName,
    pub rank: u8
}

impl Default for Perk {
    fn default() -> Self {
        Perk { perk: PerkName::Empty, rank: 0 }
    }
}

impl PartialEq<PerkRankValues> for Perk {
    fn eq(&self, other: &PerkRankValues) -> bool {
        self.perk == other.perk && self.rank == other.rank
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
            perk: x.perk,
            rank: x.rank
        }
    }
}

impl Perk {
    pub fn is_empty(&self) -> bool {
        self.perk == PerkName::Empty
    }
}