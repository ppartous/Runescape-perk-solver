use crate::{Perk, PerkRankValues};

#[derive(Debug)]
pub struct Gizmo {
    pub perks: (Perk, Perk),
    pub cost: i16,
    pub probability: f64
}

impl Default for Gizmo {
    fn default() -> Self {
        Gizmo {
            perks: (Perk::default(), Perk::default()),
            cost: 0,
            probability: 0.0
        }
    }
}

impl Gizmo {
    pub fn eq(&self, other: &Self) -> bool {
        (self.perks.0 == other.perks.0 && self.perks.1 == other.perks.1)
        || (self.perks.1 == other.perks.0 && self.perks.0 == other.perks.1)
    }

    pub fn fuzzy_eq(&self, other: &Self) -> bool {
        self.perks.0 == other.perks.0 || self.perks.1 == other.perks.0
    }

    pub fn create(x: &PerkRankValues, y: Option<&PerkRankValues>) -> Gizmo {
        Gizmo {
            perks: (
                Perk {
                    perk: x.perk,
                    rank: x.rank
                },
                if let Some(y) = y {
                    Perk {
                        perk: y.perk,
                        rank: y.rank
                    }
                } else {
                    Perk { ..Default::default() }
                }
            ),
            cost: (x.cost + if let Some(y) = y { y.cost } else { 0 }) as i16,
            probability: 0.0
        }
    }

    pub fn create_from_doubleslot(x: &PerkRankValues, y: Option<&PerkRankValues>) -> Gizmo {
        Gizmo {
            perks: (
                Perk {
                    perk: x.perk,
                    rank: x.rank
                },
                Perk { ..Default::default() }
            ),
            cost: (x.cost + if let Some(y) = y { y.cost } else { 0 }) as i16,
            probability: 0.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PerkName;

    #[test]
    fn one_perk_equal() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), true);
    }

    #[test]
    fn one_perk_not_equal_but_same_rank() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), false);
    }

    #[test]
    fn one_perk_equal_but_not_same_rank() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                Perk { perk: PerkName::Empty, rank: 0, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), false);
    }

    #[test]
    fn two_perks_equal_same_order() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), true);
    }

    #[test]
    fn two_perks_equal_not_same_order() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), true);
    }

    #[test]
    fn two_perks_equal_perks_not_same_ranks() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 3, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), false);
    }

    #[test]
    fn two_perks_not_equal_perks_same_ranks() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Equilibrium, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.eq(&y), false);
    }

    #[test]
    fn fuzzy_match_first_perk() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.fuzzy_eq(&y), true);
    }

    #[test]
    fn fuzzy_match_second_perk() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
                Perk { ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.fuzzy_eq(&y), true);
    }

    #[test]
    fn fuzzy_match_none() {
        let x = Gizmo {
            perks: (
                Perk { perk: PerkName::Biting, rank: 2, ..Default::default() },
                Perk { perk: PerkName::Precise, rank: 1, ..Default::default() },
            ),
            ..Default::default()
        };
        let y = Gizmo {
            perks: (
                Perk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                Perk { ..Default::default() },
            ),
            ..Default::default()
        };
        assert_eq!(x.fuzzy_eq(&y), false);
    }
}