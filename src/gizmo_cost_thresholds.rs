use crate::definitions::*;

pub fn find_gizmo_cost_thresholds(combination: &RankCombination, max_range: u16) -> Vec<Gizmo> {
    let mut cost_thresholds = vec![Gizmo { cost: -1, ..Default::default() }];
    let mut first_non_zero_rank_index = 0;

    let mut comb_iter = combination.ranks.iter().enumerate().peekable();
    while let Some((i, prv)) = comb_iter.next() {
        if prv.rank == 0 {
            first_non_zero_rank_index += 1;
            continue;
        }

        if prv.cost >= max_range {
            break;
        }

        if cost_thresholds.last().unwrap().cost == prv.cost as i16 {
            cost_thresholds.pop();
        }

        cost_thresholds.push(Gizmo::create(prv, None));

        let next_threshold = if let Some(x) = comb_iter.peek() { x.1.cost } else { max_range };
        for prv_two in combination.ranks.iter().take(i).skip(first_non_zero_rank_index) {
            let cost_sum = prv.cost + prv_two.cost;
            if cost_sum >= next_threshold {
                break;
            }

            if cost_thresholds.last().unwrap().cost == cost_sum as i16 {
                cost_thresholds.pop();
            }

            if prv.doubleslot || prv_two.doubleslot {
                cost_thresholds.push(Gizmo::create_from_doubleslot(prv, Some(prv_two)));
            } else {
                cost_thresholds.push(Gizmo::create(prv, Some(prv_two)));
            }
        }
    }

    cost_thresholds
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_gcth_eq(actual: &Vec<Gizmo>, expected: &Vec<Gizmo>) {
        assert!(actual.len() == expected.len(), "Actual and expected have different sizes (actual: {}, expected: {})", actual.len(), expected.len());

        for (i, (acc, exp)) in actual.iter().zip(expected).enumerate() {
            assert!(acc.cost == exp.cost, "Actual and expected have different 'cost' values at index {} (actual: {}, expected: {})", i, acc.cost, exp.cost);
            assert!(acc.perks.0.perk == exp.perks.0.perk, "Actual and expected have different 'perks.0.perk' values at index {} (actual: {}, expected: {})", i, acc.perks.0.perk, exp.perks.0.perk);
            assert!(acc.perks.1.perk == exp.perks.1.perk, "Actual and expected have different 'perks.1.perk' values at index {} (actual: {}, expected: {})", i, acc.perks.1.perk, exp.perks.1.perk);
            assert!(acc.perks.0.rank == exp.perks.0.rank, "Actual and expected have different 'perks.0.rank' values at index {} (actual: {}, expected: {})", i, acc.perks.0.rank, exp.perks.0.rank);
            assert!(acc.perks.1.rank == exp.perks.1.rank, "Actual and expected have different 'perks.1.rank' values at index {} (actual: {}, expected: {})", i, acc.perks.1.rank, exp.perks.1.rank);
        }
    }

    mod find_gizmo_cost_thresholds_tests {
        use super::*;
        use smallvec::smallvec;

        #[test]
        fn all_rank_zero() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 0, cost: 10, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 0, cost: 10, perk: PerkName::Equilibrium, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn secondary_above_maxrange() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 30, perk: PerkName::Equilibrium, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 80, perk: PerkName::Aftershock, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Biting, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { perk: PerkName::Biting, rank: 1 }), cost: 50, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { ..Default::default() }), cost: 80, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn primary_above_maxrange() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 79, perk: PerkName::Equilibrium, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 100, perk: PerkName::Aftershock, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Biting, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { ..Default::default() }), cost: 79, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { perk: PerkName::Biting, rank: 1 }), cost: 99, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn all_above_maxrange() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 30, perk: PerkName::Equilibrium, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, perk: PerkName::Aftershock, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 10);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn two_equal_costs() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 1, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Equilibrium, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, perk: PerkName::Aftershock, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Precise, rank: 1 }, Perk { ..Default::default() }), cost: 10, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { perk: PerkName::Precise, rank: 1 }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { perk: PerkName::Biting, rank: 1 }), cost: 40, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { perk: PerkName::Precise, rank: 1 }), cost: 70, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { perk: PerkName::Equilibrium, rank: 1 }), cost: 80, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn three_equal_costs_excluding_first() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 1, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Equilibrium, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Aftershock, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, perk: PerkName::Mobile, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Precise, rank: 1 }, Perk { ..Default::default() }), cost: 10, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { perk: PerkName::Precise, rank: 1 }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { perk: PerkName::Equilibrium, rank: 1 }), cost: 40, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Mobile, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Mobile, rank: 1 }, Perk { perk: PerkName::Precise, rank: 1 }), cost: 70, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Mobile, rank: 1 }, Perk { perk: PerkName::Aftershock, rank: 1 }), cost: 80, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn perk_is_doubleslot() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 1, cost: 10, perk: PerkName::Precise, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, perk: PerkName::Biting, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 35, perk: PerkName::Equilibrium, doubleslot: true, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, perk: PerkName::Aftershock, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Precise, rank: 1 }, Perk { ..Default::default() }), cost: 10, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Biting, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Biting, rank: 1 }, Perk { perk: PerkName::Precise, rank: 1 }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { ..Default::default() }), cost: 35, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { ..Default::default() }), cost: 45, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Equilibrium, rank: 1 }, Perk { ..Default::default() }), cost: 55, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { perk: PerkName::Precise, rank: 1 }), cost: 70, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { perk: PerkName::Biting, rank: 1 }), cost: 80, ..Default::default() },
                Gizmo { perks: (Perk { perk: PerkName::Aftershock, rank: 1 }, Perk { ..Default::default() }), cost: 95, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }
    }
}