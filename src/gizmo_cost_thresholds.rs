use crate::prelude::*;
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

pub fn find_gizmo_cost_thresholds(combination: &RankCombination, max_range: u16) -> Vec<Gizmo> {
    let mut cost_thresholds = vec![Gizmo {
        cost: -1,
        ..Default::default()
    }];
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

        let next_threshold = if let Some(x) = comb_iter.peek() {
            x.1.cost
        } else {
            max_range
        };
        for prv_two in combination
            .ranks
            .iter()
            .take(i)
            .skip(first_non_zero_rank_index)
        {
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

/// This function is used when the perks in the gizmo we are looking for are known
/// and the order of the perks doesn't matter
pub fn find_wanted_gizmo_cost_thresholds(
    combination: &RankCombination,
    max_range: u16,
    wanted_gizmo: Gizmo,
) -> SmallVec<[Gizmo; 5]> {
    let mut cost_thresholds = smallvec![];
    let mut first_non_zero_rank_index = 0;
    let mut perk_two_index = None;
    let mut double_slot_locations = VecDeque::new();

    /*
        Ex: We have 4 perks with the following costs
        P1 = 200
        p2 = 100
        p3 = 50
        p4 = 30

        If we want perks P2P3

            P4        P2     P2P3  P1     P1P3             max_range
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
                             |-----|                        <-- Region we need to find

        If we wanted P1P3 instead we would get

            P4        P2     P2P3  P1     P1P3             max_range
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
                                          |-----|           <-- Regions we need to find

        If a perk in the second slot is a doubleslot perk then it will delete itself.
        So if we wanted just P2 and P3 is a doubleslot perk we need to get

            P4        P2     P2P3  P1     P1P3             max_range
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
                      |---|  |-----|                        <-- Region we need to find
    */

    let mut comb_iter = combination.ranks.iter().enumerate().peekable();
    while let Some((i, prv)) = comb_iter.next() {
        if prv.rank == 0 {
            first_non_zero_rank_index += 1;
            continue;
        }

        if prv.doubleslot && wanted_gizmo.perks.1.is_empty() {
            // We need these in case we only want a single perk but there are doubleslot perks with a lower index because
            // doubleslot perks in the second slot of the gizmo will delete themself so the result is again the single perk we want
            double_slot_locations.push_back(i);
        }

        if !(*prv == wanted_gizmo.perks.0 || *prv == wanted_gizmo.perks.1) {
            continue;
        }

        if prv.cost >= max_range {
            break;
        }

        let mut perk_one = *prv;
        let mut perk_two = PerkRankValues {
            ..Default::default()
        };

        if wanted_gizmo.perks.1.is_empty() {
            // Singular perk can't exist if there is a higher index perk with equal cost
            if let Some((_, x)) = comb_iter.peek() {
                if prv.cost == x.cost {
                    break;
                }
            }

            cost_thresholds.push(Gizmo::create(prv, None));
            let next_major_threshold = if let Some((_, x)) = comb_iter.peek() {
                x.cost
            } else {
                max_range
            };

            if prv.doubleslot {
                if let Some((_, x)) = comb_iter.peek() {
                    double_slot_locations.pop_front();
                    perk_one = **x;
                } else {
                    // Doubleslot perks delete the second perk so if the wanted perk is the last in the list than the next
                    // threshold is max_range
                    break;
                }
            } else if first_non_zero_rank_index == i
                || (prv.cost + combination.ranks[first_non_zero_rank_index].cost
                    >= next_major_threshold)
            {
                if let Some((_, x)) = comb_iter.peek() {
                    perk_one = **x;
                } else {
                    break;
                }
            } else {
                for x in combination.ranks.iter().skip(first_non_zero_rank_index) {
                    if x.doubleslot {
                        double_slot_locations.pop_front();
                    } else {
                        perk_two = *x;
                        break;
                    }
                }
            }

            if perk_one.cost + perk_two.cost < max_range {
                cost_thresholds.push(Gizmo::create(&perk_one, Some(&perk_two)));
            } else {
                break;
            }

            while !double_slot_locations.is_empty() {
                let double_loc = double_slot_locations.pop_front().unwrap();

                perk_two = combination.ranks[double_loc];
                if prv.cost + perk_two.cost < next_major_threshold {
                    cost_thresholds.push(Gizmo::create_from_doubleslot(prv, Some(&perk_two)));
                } else {
                    break;
                }

                let mut end_index = double_loc + 1;
                while let Some(x) = double_slot_locations.front() {
                    if *x == end_index {
                        end_index += 1;
                        double_slot_locations.pop_front();
                    } else {
                        break;
                    }
                }

                if end_index == i {
                    if let Some((_, x)) = comb_iter.peek() {
                        perk_one = **x;
                        perk_two = PerkRankValues {
                            ..Default::default()
                        };
                    } else {
                        break;
                    }
                } else {
                    perk_one = *prv;
                    perk_two = combination.ranks[end_index];
                }

                if perk_one.cost + perk_two.cost < max_range {
                    cost_thresholds.push(Gizmo::create(&perk_one, Some(&perk_two)));
                } else {
                    break;
                }
            }

            break;
        } else if perk_two_index.is_none() {
            perk_two_index = Some(i);
        } else {
            let perk_two_index = perk_two_index.unwrap();
            let next_major_threshold = if let Some((_, x)) = comb_iter.peek() {
                x.cost
            } else {
                max_range
            };
            perk_two = combination.ranks[perk_two_index];

            if perk_one.cost + perk_two.cost >= next_major_threshold {
                break;
            }

            if perk_two_index < i - 1 && combination.ranks[perk_two_index + 1].cost == perk_two.cost
            {
                break;
            }

            cost_thresholds.push(Gizmo::create(&perk_one, Some(&perk_two)));

            if perk_two_index < combination.ranks.len() - 2 {
                // perk_two is lower than P2 so there is always a higher threshold
                if (i < combination.ranks.len() - 1) // perk_one is lower than P1
                    && ((perk_two_index == i - 1) || (perk_one.cost + combination.ranks[perk_two_index + 1].cost >= next_major_threshold))
                {
                    perk_one = combination.ranks[i + 1];
                    perk_two = PerkRankValues {
                        ..Default::default()
                    };
                } else {
                    perk_one = *prv;
                    perk_two = combination.ranks[perk_two_index + 1];
                }

                if perk_one.cost + perk_two.cost < max_range {
                    cost_thresholds.push(Gizmo::create(&perk_one, Some(&perk_two)));
                }
            }

            break;
        }
    }

    cost_thresholds
}

/// This function is used for fuzzy search where the wanted perk can be combined with any other perk
/// and the order of the perks doesn't matter so our wanted perk can be in slot 1 or 2
pub fn fuzzy_find_wanted_gizmo_cost_thresholds(
    combination: &RankCombination,
    max_range: u16,
    wanted_gizmo: Gizmo,
) -> SmallVec<[Gizmo; 5]> {
    let mut cost_thresholds = smallvec![];

    /*
        Ex: We have 4 perks with the following costs
        P1 = 200
        P2 = 100
        P3 = 50
        P4 = 30

        If we want perk P3

            P4        P2     P2P3  P1     P1P3             maxRange
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
               |------|      |-----|      |-----|           <-- Regions we need to find

        If we want P3 and P3 is a doubleslot

            P4        P2     P2P3  P1     P1P3             maxRange
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
               |------|                                     <-- Regions we need to find

        If we want P3 and P2 is doubleslot

            P4        P2     P2P3  P1     P1P3             maxRange
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
               |------|                   |-----|           <-- Regions we need to find

        If we wanted P4 instead we would get

            P4        P2     P2P3  P1     P1P3             maxRange
        |---|--|---|--|---|--|-----|---|--|-----|----------|
               P3  P3P4   P2P4         P1P4     P1P2
            |--|   |--|   |--|         |--|                 <-- Regions we need to find
    */

    for (i, values) in combination
        .ranks
        .iter()
        .zip_longest(combination.ranks.iter().skip(1))
        .enumerate()
        .rev()
    {
        let prv = values.clone().left().unwrap();
        let prv_next = values.right(); // 1 index higher than prv

        if *prv != wanted_gizmo.perks.0 {
            continue;
        }

        if prv.cost >= max_range {
            break;
        }

        // Find borders for when wanted perk is in gizmo slot one
        if !(prv_next.is_some() && prv_next.unwrap().cost == prv.cost) {
            cost_thresholds.push(Gizmo::create(prv, None));

            if let Some(prv_next) = prv_next {
                if prv_next.cost < max_range {
                    cost_thresholds.push(Gizmo::create(prv_next, None));
                }
            }
        }

        if prv.doubleslot {
            break;
        }

        // Find borders for when wanted perk is in gizmo slot two
        // All perks with a higher index than prv can potentially create a wanted gizmo
        for (j, values2) in combination
            .ranks
            .iter()
            .zip_longest(combination.ranks.iter().skip(1))
            .enumerate()
            .skip(i + 1)
        {
            let prv_two = values2.clone().left().unwrap(); // prv_two is in gizmo slot one
            let prv_two_next = values2.right();
            let next_major_threshold = if let Some(x) = prv_two_next {
                u16::min(x.cost, max_range)
            } else {
                max_range
            };

            // When the costs of perk one and perk two are equal then they can only form a gizmo if they are next to each other
            if i < j - 1 && prv.cost == prv_next.unwrap().cost {
                break;
            }

            if (prv.cost + prv_two.cost) >= next_major_threshold || prv_two.doubleslot {
                continue;
            }

            if (prv.cost + prv_two.cost) < max_range {
                cost_thresholds.push(Gizmo::create(prv_two, Some(prv)));
            } else {
                break;
            }

            let next_threshold = prv_two.cost + prv_next.unwrap().cost; // Ex: wanted is P4 so i+1 is P3. perkTwo points to P2. So the range is from P2P4 to P2P3
            if j == i + 1 // If j points to 1 perk higher than our wanted perk (e.g. P3P4 when wanted is P4) then the next threshold will always be i+2 (P2 when wanted is P4)
                || next_threshold >= next_major_threshold
            // Ex: Range is from P2P4 to P2P3 but P2P3 cost more than P1 so range is actually P2P4 to P1
            {
                // If j = P1 than the next threshold is maxRange instead of another perk
                if prv_two_next.is_some() && next_major_threshold < max_range {
                    cost_thresholds.push(Gizmo::create(prv_two_next.unwrap(), None));
                } else {
                    break;
                }
            } else {
                cost_thresholds.push(Gizmo::create(prv_two, Some(prv_next.unwrap())));
            }
        }

        break;
    }

    cost_thresholds
}

// =====================================================================================================================
//                                                      Tests
// =====================================================================================================================

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;
    use crate::utils::{check_len_result, check_index_result};

    fn assert_gcth_eq_result(actual: &[Gizmo], expected: &[Gizmo]) -> Result<(), String> {
        check_len_result(actual, expected)?;

        for (i, (acc, exp)) in actual.iter().zip(expected).enumerate() {
            check_index_result(acc.perks.0.name, exp.perks.0.name, i, "perks.0.name", actual, expected)?;
            check_index_result(acc.perks.1.name, exp.perks.1.name, i, "perks.1.name", actual, expected)?;
            check_index_result(acc.perks.0.rank, exp.perks.0.rank, i, "perks.0.rank", actual, expected)?;
            check_index_result(acc.perks.1.rank, exp.perks.1.rank, i, "perks.1.rank", actual, expected)?;
            check_index_result(acc.cost, exp.cost, i, "cost", actual, expected)?;
        }
        Ok(())
    }

    fn assert_gcth_eq(actual: &[Gizmo], expected: &[Gizmo]) {
        if let Err(err) = assert_gcth_eq_result(actual, expected) {
            panic!("{}", err);
        }
    }

    mod find_gizmo_cost_thresholds_tests {
        use super::*;

        #[test]
        fn all_rank_zero() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 0, cost: 10, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 0, cost: 10, name: PerkName::C, ..Default::default() },
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
                    PerkRankValues { rank: 0, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 30, name: PerkName::C, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 80, name: PerkName::D, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 50, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 80, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn primary_above_maxrange() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 79, name: PerkName::C, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 100, name: PerkName::D, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 79, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 99, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn all_above_maxrange() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 0, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 30, name: PerkName::C, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, name: PerkName::D, ..Default::default() },
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
                    PerkRankValues { rank: 1, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::C, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, name: PerkName::D, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::A, rank: 1 }, Perk { ..Default::default() }), cost: 10, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 40, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 70, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 80, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn three_equal_costs_excluding_first() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 1, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::C, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::D, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, name: PerkName::E, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::A, rank: 1 }, Perk { ..Default::default() }), cost: 10, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 40, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 70, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { name: PerkName::D, rank: 1 }), cost: 80, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn perk_is_doubleslot() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { rank: 1, cost: 10, name: PerkName::A, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 20, name: PerkName::B, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 35, name: PerkName::C, doubleslot: true, ..Default::default() },
                    PerkRankValues { rank: 1, cost: 60, name: PerkName::D, ..Default::default() },
                ],
                probability: 0.0,
            };
            let expected = vec![
                Gizmo { cost: -1, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::A, rank: 1 }, Perk { ..Default::default() }), cost: 10, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 35, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 45, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 55, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 70, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 95, ..Default::default() },
            ];
            let actual = find_gizmo_cost_thresholds(&combination, 100);
            assert_gcth_eq(&actual, &expected);
        }
    }

    mod find_wanted_gizmo_cost_thresholds_tests {
        use super::*;

        #[test]
        fn all_rank_zero() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 0, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 0, cost: 10, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::A, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn single_wanted_primary_cutoff() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        mod single_wanted_secondary_cutoff {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_100() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn max_range_70() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 70, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod single_wanted_max_range_cutoff {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_51() {
                let (combination, wanted_gizmo) = setup();
                let expected1 = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 50, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 51, wanted_gizmo);
                assert_gcth_eq(&actual, &expected1);
            }

            #[test]
            fn max_range_50() {
                let (combination, wanted_gizmo) = setup();
                let expected2 = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 50, wanted_gizmo);
                assert_gcth_eq(&actual, &expected2);
            }
        }

        #[test]
        fn single_wanted_one_combo_possible() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        mod single_wanted_perk_two_equal_costs {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn possible_lower_duplicate() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 30, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn impossible_lower_duplicate() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn possible_high_duplicate() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 70, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn impossible_high_duplicate() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod single_wanted_perk_three_equal_costs {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 30, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn impossible_1() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn impossible_2() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod two_wanted_perks_three_equal_costs_next_is_more_than_double {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 40, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn impossible_1() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn impossible_2() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        #[test]
        fn two_wanted_perks_three_equal_costs_next_is_not_more_than_double() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::E, rank: 1, cost: 40, ..Default::default() },
                ],
                probability: 0.0,
            };

            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), ..Default::default() };
            let expected = vec![];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn single_wanted_is_double_slot_not_first_inline() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn single_wanted_is_double_slot_not_first_inline_secondary_is_double_slot() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn single_wanted_is_double_slot_first_inline() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, doubleslot: true, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn single_wanted_second_is_double_slot() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        mod single_wanted_second_is_double_slot_secondary_threshold_too_high {
            use super::*;

            #[test]
            fn a_as_secondary_not_possible() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 35, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 35, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn a_as_secondary_possible() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 45, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 40, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod single_wanted_multiple_second_are_double_slot {
            use super::*;

            #[test]
            fn two_non_consecutive() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 90, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn three_consecutive() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn two_consecutive_plus_one() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::L, rank: 1, cost: 5, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 100, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 90, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 100, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 200, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn one_lower_than_wanted_is_not_doubleslot() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 5, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::F, rank: 1, cost: 200, ..Default::default() },
                        PerkRankValues { name: PerkName::G, rank: 1, cost: 350, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::H, rank: 1, cost: 700, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::I, rank: 1, cost: 1400, ..Default::default() },
                        PerkRankValues { name: PerkName::J, rank: 1, cost: 2800, ..Default::default() },
                        PerkRankValues { name: PerkName::L, rank: 1, cost: 6000, doubleslot: true, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 2800, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 2820, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 2830, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::E, rank: 1 }), cost: 2860, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 3150, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::I, rank: 1 }), cost: 4200, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 10000, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn one_lower_than_wanted_is_doubleslot_and_wanted_is_not_last() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 5, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::F, rank: 1, cost: 200, ..Default::default() },
                        PerkRankValues { name: PerkName::G, rank: 1, cost: 350, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::H, rank: 1, cost: 700, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::I, rank: 1, cost: 1400, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::J, rank: 1, cost: 2800, ..Default::default() },
                        PerkRankValues { name: PerkName::L, rank: 1, cost: 6000, doubleslot: true, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 2800, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 2820, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 2830, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::E, rank: 1 }), cost: 2860, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 3150, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::L, rank: 1 }, Perk { ..Default::default() }), cost: 6000, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 10000, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn one_lower_than_wanted_is_doubleslot_and_wanted_is_last() {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 5, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 10, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 30, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::F, rank: 1, cost: 200, ..Default::default() },
                        PerkRankValues { name: PerkName::G, rank: 1, cost: 350, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::H, rank: 1, cost: 700, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::I, rank: 1, cost: 1400, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::J, rank: 1, cost: 2800, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 2800, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 2820, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 2830, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { name: PerkName::E, rank: 1 }), cost: 2860, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::J, rank: 1 }, Perk { ..Default::default() }), cost: 3150, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 10000, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        #[test]
        fn double_wanted_primary_cutoff() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), ..Default::default() };
            let wanted_gizmo_inv = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 50, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo_inv);
            assert_gcth_eq(&actual, &expected);
        }

        #[test]
        fn double_wanted_secondary_cutoff() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                    PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
            let wanted_gizmo_inv = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { name: PerkName::D, rank: 1 }), ..Default::default() };
            let expected = vec![
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 90, ..Default::default() },
            ];
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
            let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo_inv);
            assert_gcth_eq(&actual, &expected);
        }

        mod double_wanted_max_range_cutoff {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_larger_than_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 90, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 91, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn max_range_equal_to_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 90, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod double_wanted_secondary_above_max_range {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 80, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_larger_than_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 100, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 101, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn max_range_equal_to_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod double_wanted_primary_above_max_range {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 80, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_larger_than_primary_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 81, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn max_range_equal_to_primary_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 80, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod double_wanted_secondary_above_next_primary {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 9, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 50, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 75, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn sum_equal_to_next_primary() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn sum_one_less_than_next_primary() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::A, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { name: PerkName::A, rank: 1 }), cost: 29, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn sum_next_minor_larger_than_next_major() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 70, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 75, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod double_wanted_secondary_one_of_two_equal_cost {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn secondary_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 80, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn secondary_not_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod double_wanted_primary_one_of_two_equal_cost {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn primary_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 120, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 150, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn primary_not_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 150, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod double_wanted_primary_and_secondary_equal_cost {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn primary_and_secondary_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 120, ..Default::default() },
                ];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 150, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn secondary_not_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 150, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn primary_and_secondary_not_possible() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), ..Default::default() };
                let expected = vec![];
                let actual = find_wanted_gizmo_cost_thresholds(&combination, 150, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }
    }

    mod fuzzy_find_wanted_gizmo_cost_thresholds_tests {
        use super::*;

        #[test]
        fn all_rank_zero() {
            let combination = RankCombination {
                ranks: smallvec![
                    PerkRankValues { name: PerkName::A, rank: 0, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::B, rank: 0, cost: 10, ..Default::default() },
                    PerkRankValues { name: PerkName::C, rank: 0, cost: 10, ..Default::default() },
                ],
                probability: 0.0,
            };
            let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
            let expected = vec![];
            let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
            assert_gcth_eq(&actual, &expected);
        }

        mod wanted_as_primary_above_max_range {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_equal_to_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 30, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn max_range_larger_than_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 31, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod wanted_as_secondary_above_max_range {
            use super::*;

            fn setup() -> (RankCombination, Gizmo) {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                (combination, wanted_gizmo)
            }

            #[test]
            fn max_range_equal_to_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 90, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn max_range_larger_than_cost() {
                let (combination, wanted_gizmo) = setup();
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 90, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 91, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod all_below_max_range {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 5, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 30, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_p1() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_p2() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 90, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_p3() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 30, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 50, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 80, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 90, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod two_equal_costs_next_is_more_than_double {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_first_of_double() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 80, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_second_of_double() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { name: PerkName::B, rank: 1 }), cost: 40, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod two_equal_costs_next_is_not_more_than_double {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 40, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_first_of_double() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 40, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_second_of_double() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod three_equal_costs_excluding_first_next_is_more_than_double {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_is_first_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { name: PerkName::D, rank: 1 }), cost: 80, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_second_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { name: PerkName::C, rank: 1 }), cost: 40, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_third_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod three_equal_costs_excluding_first_next_is_not_more_than_double {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 40, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_is_first_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 40, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { name: PerkName::D, rank: 1 }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_second_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_third_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod three_equal_costs_including_first {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, ..Default::default() },
                        PerkRankValues { name: PerkName::E, rank: 1, cost: 60, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_is_first_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 200, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_second_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::E, rank: 1 }, Perk { name: PerkName::D, rank: 1 }), cost: 120, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 200, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_third_of_triple() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 200, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }
        }

        mod wanted_is_double_slot {
            use super::*;

            fn setup() -> RankCombination {
                let combination = RankCombination {
                    ranks: smallvec![
                        PerkRankValues { name: PerkName::A, rank: 1, cost: 10, ..Default::default() },
                        PerkRankValues { name: PerkName::B, rank: 1, cost: 20, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::C, rank: 1, cost: 60, doubleslot: true, ..Default::default() },
                        PerkRankValues { name: PerkName::D, rank: 1, cost: 60, doubleslot: true, ..Default::default() },
                    ],
                    probability: 0.0,
                };
                combination
            }

            #[test]
            fn wanted_is_p1() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_p2() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected = vec![];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                assert_gcth_eq(&actual, &expected);
            }

            #[test]
            fn wanted_is_p3() {
                let combination = setup();
                let wanted_gizmo = Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), ..Default::default() };
                let expected1 = vec![
                    Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::D, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let expected2 = vec![
                    Gizmo { perks: (Perk { name: PerkName::B, rank: 1 }, Perk { ..Default::default() }), cost: 20, ..Default::default() },
                    Gizmo { perks: (Perk { name: PerkName::C, rank: 1 }, Perk { ..Default::default() }), cost: 60, ..Default::default() },
                ];
                let actual = fuzzy_find_wanted_gizmo_cost_thresholds(&combination, 100, wanted_gizmo);
                let result1 = assert_gcth_eq_result(&actual, &expected1);
                let result2 = assert_gcth_eq_result(&actual, &expected2);
                if result1.is_err() && result2.is_err() {
                    panic!("{}\n\n{}", result1.unwrap_err(),  result2.unwrap_err());
                }
            }
        }
    }
}
