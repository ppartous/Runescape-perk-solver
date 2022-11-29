use std::collections::HashMap;
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};
use crate::{definitions::*, dice, utils};

/// Calculate the base and roll values for each possible perk based on the input materials and their order.
pub fn get_perk_values(data: &Data, input_materials: &Vec<MaterialName>, gizmo_type: GizmoType,
    is_ancient_gizmo: bool) -> Vec<PerkValues> {

    let mut perk_values_order = Vec::new();
    let mut possible_perks: HashMap<PerkName, PerkValues> = HashMap::new();

    for mat in input_materials {
        let mat_data = &data.comps[mat][gizmo_type];
        let is_ancient_mat = data.comps[mat].ancient_only;

        if is_ancient_mat && !is_ancient_gizmo {
            continue;
        }

        for perk_data in mat_data {
            let mut perk_roll = perk_data.roll;
            let mut perk_base = perk_data.base;

            if is_ancient_gizmo && !is_ancient_mat {
                perk_roll = (perk_roll * 8) / 10;
                perk_base = (perk_base * 8) / 10;
            }

            if possible_perks.contains_key(&perk_data.perk) {
                let x = possible_perks.get_mut(&perk_data.perk).unwrap();
                x.rolls.push(perk_roll as u8);
                x.base += perk_base;
            } else {
                // Only set some values as we need to check first if this material combination can even generate the rank that we need
                let perk_values = PerkValues {
                    perk: perk_data.perk,
                    base: perk_base,
                    rolls: smallvec![perk_roll as u8],
                    ..Default::default()
                };

                possible_perks.insert(perk_data.perk, perk_values);
                perk_values_order.push(perk_data.perk);
            }
        }
    }

    let mut perk_values_arr = Vec::with_capacity(possible_perks.len());
    for name in perk_values_order {
        let x = possible_perks.remove(&name).unwrap();
        perk_values_arr.push(x);
    }

    perk_values_arr
}

pub fn calc_perk_rank_probabilities(data: &Data, perk_values_arr: &mut Vec<PerkValues>, is_ancient_gizmo: bool) {
    for perk_values in perk_values_arr.iter_mut() {
        let perk_data = &data.perks[&perk_values.perk];

        for x in perk_data.ranks.iter() {
            perk_values.ranks.push(PerkRankValuesProbabilityContainer { values: *x, probability: 0.0 });
        }

        perk_values.i_first = 0;
        perk_values.i_last = perk_values.ranks.len() - 1;
        perk_values.doubleslot = perk_data.doubleslot;

        perk_values.rolls.sort();
        let mut roll_dist = Vec::new();

        let mut iter = perk_values.rolls.iter().peekable();
        let mut count = 1;
        while let Some(x) = iter.next() {
            while let Some(next) = iter.peek() {
                if *next != x { break; }
                count += 1;
                iter.next();
            }

            if roll_dist.len() > 0 {
                roll_dist = utils::convolve(&roll_dist, &dice::get_distribution(*x as usize, count));
            } else {
                roll_dist = dice::get_distribution(*x as usize, count);
            }
            count = 1;
        }

        for i in 0..(perk_values.ranks.len()) {
            let mut next_threshold = roll_dist.len() as i64 - 1;

            if i + 1 < perk_values.ranks.len() {
                let next_container = unsafe{ perk_values.ranks.get_unchecked(i + 1)};
                if !(next_container.values.ancient_only && !is_ancient_gizmo) {
                    // -1 because the range we need ends right before the threshold value of the next perk
                    next_threshold = next_threshold.min(next_container.values.threshold as i64 - perk_values.base as i64 - 1);
                }
            }

            let mut container = unsafe{ perk_values.ranks.get_unchecked_mut(i) };
            let range_start = i64::max(container.values.threshold as i64 - perk_values.base as i64, 0) as usize;
            if !(container.values.ancient_only && !is_ancient_gizmo) && next_threshold >= 0 && (range_start as i64) < next_threshold {
                container.probability = roll_dist[range_start..=next_threshold as usize].iter().sum();
            }

            if container.probability == 0.0 {
                if perk_values.i_first == i {
                    perk_values.i_first += 1;
                } else if perk_values.i_last >= i {
                    perk_values.i_last = i - 1;
                }
            }
        }
    }
}

/// Quick check if it is even possible to generate the wanted perk rank. This won't catch all impossible material orders.
pub fn can_generate_wanted_ranks(data: &Data, perk_values_arr: &Vec<PerkValues>, wanted_gizmo: &WantedGizmo) -> bool {
    let wanted_rank1 = wanted_gizmo.0.rank as usize;
    let wanted_rank2 = wanted_gizmo.1.rank as usize;

    let perk1_ranks = &data.perks[&wanted_gizmo.0.perk].ranks;
    let perk2_ranks = &data.perks[&wanted_gizmo.1.perk].ranks;

    let perk1_threshold = perk1_ranks[wanted_rank1].threshold as usize;
    let perk1_next_threshold = if wanted_rank1 + 1 < perk1_ranks.len() { perk1_ranks[wanted_rank1 + 1].threshold as usize } else { usize::MAX };

    let perk2_threshold = perk2_ranks[wanted_rank2].threshold as usize;
    let perk2_next_threshold = if wanted_rank2 + 1 < perk2_ranks.len() { perk2_ranks[wanted_rank2 + 1].threshold as usize } else { usize::MAX };

    let mut perk1_base = None;
    let mut perk1_max_roll = None;
    let mut perk2_base = None;
    let mut perk2_max_roll = None;

    for perk_value in perk_values_arr {
        if perk_value.perk == wanted_gizmo.0.perk {
            perk1_base = Some(perk_value.base as usize);
            perk1_max_roll = Some(perk_value.rolls.iter().map(|x| *x as usize).sum::<usize>() - perk_value.rolls.len());
        }
        if perk_value.perk == wanted_gizmo.1.perk {
            perk2_base = Some(perk_value.base as usize);
            perk2_max_roll = Some(perk_value.rolls.iter().map(|x| *x as usize).sum::<usize>() - perk_value.rolls.len());
        }
    }

    if perk1_base == None || (wanted_gizmo.1.perk != PerkName::Empty && perk2_base == None) {
        return false;
    } else if !(perk1_base.unwrap() + perk1_max_roll.unwrap() >= perk1_threshold && perk1_base.unwrap() < perk1_next_threshold) {
        return false;
    } else if wanted_gizmo.1.perk != PerkName::Empty
    && !(perk2_base.unwrap() + perk2_max_roll.unwrap() >= perk2_threshold && perk2_base.unwrap() < perk2_next_threshold) {
        return false;
    }

    true
}

pub fn permutate_perk_ranks(perk_list: &Vec<PerkValues>, wanted_gizmo: Option<&WantedGizmo>) -> Vec<RankCombination> {
    let mut combinations = Vec::with_capacity(perk_list.len() * 7);

    for pv_combination in perk_list.iter().map(|x| {
        let mut i_first = x.i_first;
        let mut i_last = x.i_last;
        match wanted_gizmo {
            Some(gizmo) if gizmo.0.perk == x.perk => {
                i_first = gizmo.0.rank as usize;
                i_last = i_first;
            },
            Some(gizmo) if gizmo.1.perk == x.perk => {
                i_first = gizmo.1.rank as usize;
                i_last = i_first;
            },
            _ => ()
        }
        x.ranks.iter().skip(i_first).take(i_last - i_first + 1)
    }).multi_cartesian_product() {

        let mut probability = 1.0;
        let mut ranks = SmallVec::<[PerkRankValues; 8]>::new();

        for rank in pv_combination {
            probability *= rank.probability;
            ranks.push(rank.values);
        }

        combinations.push(RankCombination { ranks, probability });
    }

    combinations
}

/// When `permutate_perk_ranks` is called with a `wanted_gizmo` it will no longer generate all possible rank combinations
/// to improve performance so those can't be used anymore to calculate the empty gizmo chance.
///
/// # Theory of operation
/// `budget` is a probability distribution based on you invention level where the minimum value possible is equal to
/// your invention level and the maximum is 5 or 6 times the level depending on ancient gizmo of not.
/// Each `perkValue` contains a list of ranks and their probability that this rank is generated.
/// A gizmo is empty for a certain combination of ranks if all of those ranks are zero or if the budget value is smaller
/// or equal to the lowest cost value of all the ranks in that combination ignoring zero ranks. So to calculate the
/// total empty gizmo chance we need to sum the chance that all ranks are zero and the chances for each rank combination
/// where the budget value is lower or equal to the lowest cost value.
///
/// Example:
///
/// Let `X0` mean rank 0 for perk `X` and `pX0` the chance of this rank. Let `pBud(X)` means the chance that budget is
/// lower or equal to `X`.
///
/// Assume we have the following set of perk ranks and their cost values (ranks with zero probability are left out) and
/// the invention level is 25:\
/// `A0` = 0, `A1` = 10, `A2` = 30 \
/// `B1` = 20, `B2` = 50 \
/// `C4` = 40, `C5` = 45
///
/// In this case the chance that all perks are rank zero is 0. `A1` and `B1` have a cost lower than the minimum budget value
/// so any rank combination with these perk ranks can't create an empty gizmo. So the total empty gizmo chance becomes:\
/// `p_empty` = `pA2 * pB2 * (pC4 + pC5) * pBud(30) + pA0 * pB2 * pC4 * pBud(40) + pA0 * pB2 * pC5 * pBud(45)`
///
/// ## Algorithm
/// First calculate the chance that to get a combination of ranks that can generate an empty gizmo. In the above example
/// this would be \
/// `p_empty_combo` = `(pA0 + pA2) * pB2 * (pC4 + pC5)` \
/// Next also do this for each perk seperately, so `(pA0 + pA2)` for perk `A`, `pB2` for perk `B` and `(pC4 + pC5)` for
/// perk `C`. Then store each non zeo rank that has a cost above the minimum budget value in an array and sort it on
/// cost from low to high: `[A2, C4, C5, B2]`. Then iterate over it where on each step calculate the empty combination
/// chance that contains this specific rank by dividing the total chance by the total perk chance and multiplying by
/// the rank chance. So for `A2` this is: \
/// `((pA0 + pA2) * pB2 * (pC4 + pC5)) * pA2 / (pA0 + pA2)` = `pA2 * pB2 * (pC4 + pC5)` \
/// This is the first part of `p_empty`. After this we remove this rank from `p_empty_combo` so it becomes: \
/// `p_empty_combo` = `pA0 * pB2 * (pC4 + pC5)` \
/// and also remove it from the perk total chance: \
/// `(pA0 + pA2) - pA2` = `pA0` \
/// Then move on to do the same with the next rank in the array.
pub fn get_empty_gizmo_chance(budget: &Budget, perk_values_arr: &Vec<PerkValues>) -> f64 {
    let mut p_empty = 1.0; // Total empty gizmo chance
    let mut p_empty_per_perk = HashMap::new();
    let mut ranks = Vec::new(); // vec of non zero ranks with a cost higher than the invention level

    // Chance to have a combination of perk ranks that can produce an empty gizmo (perks with a cost higher than
    // the invention level or of rank 0)
    let mut p_empty_combo = 1.0;

    for pv in perk_values_arr.iter() {
        let pv_ranks = &pv.ranks;

        p_empty *= pv_ranks[0].probability;

        let mut psum = pv_ranks[0].probability;
        for rank in pv_ranks.iter().skip(pv.i_first).take(pv.i_last - pv.i_first + 1).rev()  {
            if rank.values.cost <= budget.range.min {
                break;
            }

            psum += rank.probability;
            ranks.push(rank);
        }

        p_empty_per_perk.insert(pv.perk, psum);
        p_empty_combo *= psum;
    }

    if p_empty_combo == 0.0 {
        return 0.0;
    } else if ranks.is_empty() {
        return p_empty;
    }

    ranks.sort_by(|x, y| x.values.cost.cmp(&y.values.cost));

    for rank in ranks {
        // Adjusted empty combo chance to a specific rank of a perk
        let mut p_empty_rank = p_empty_combo * rank.probability / p_empty_per_perk[&rank.values.perk];
        // Multiply with chance that our inventbudget is bellow the rank cost
        p_empty_rank *= budget.dist[u16::min(rank.values.cost, budget.range.max) as usize];
        p_empty += p_empty_rank;
        // Remove this rank from 'p_empty_combo'
        p_empty_combo *= (p_empty_per_perk[&rank.values.perk] - rank.probability) / p_empty_per_perk[&rank.values.perk];
        // Remove this rank from the combined empty combo chance of a certain perk
        *p_empty_per_perk.get_mut(&rank.values.perk).unwrap() -= rank.probability;

        if p_empty_combo == 0.0 {
            break;
        }
    }

    p_empty
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use lazy_static::lazy_static;
    use approx::abs_diff_eq;
    use crate::load_data;

    fn assert_perk_values_eq(actual: &Vec<PerkValues>, expected: &Vec<PerkValues>) {
        assert!(actual.len() == expected.len(), "Actual and expected have different sizes (actual: {}, expected: {})", actual.len(), expected.len());

        for (acc, exp) in actual.iter().zip(expected) {
            assert_eq!(acc.base, exp.base, "Actual and expected have different 'base' values (actual: {}, expected: {})", acc.base, exp.base);
            assert_eq!(acc.perk, exp.perk, "Actual and expected have different 'perk' values (actual: {}, expected: {})", acc.perk, exp.perk);
            assert_eq!(acc.doubleslot, exp.doubleslot, "Actual and expected have different 'doubleslot' values (actual: {}, expected: {})", acc.doubleslot, exp.doubleslot);
            assert_eq!(acc.i_first, exp.i_first, "Actual and expected have different 'i_first' values (actual: {}, expected: {})", acc.i_first, exp.i_first);
            assert_eq!(acc.i_last, exp.i_last, "Actual and expected have different 'i_last' values (actual: {}, expected: {})", acc.i_last, exp.i_last);

            for (i, (x, y))  in acc.rolls.iter().zip_eq(&exp.rolls).enumerate() {
                assert_eq!(x, y, "Actual and expected have different 'rolls' values at index {} (actual: {}, expected: {})", i, x, y);
            }

            for (i, (x, y))  in acc.ranks.iter().zip_eq(&exp.ranks).enumerate() {
                assert_eq!(x.values, y.values, "Actual and expected have different 'ranks.values' values at index {} (actual: {}, expected: {})", i, x.values, y.values);
                let res = abs_diff_eq!(x.probability, y.probability, epsilon = 8.0 * f64::EPSILON);
                assert!(res, "Actual and expected have different 'ranks.probability' values at index {} (actual: {}, expected: {})", i, x.probability, y.probability);
            }
        }
    }

    mod get_perk_values_tests {
        use super::*;

        lazy_static!{
            static ref DATA: Data = load_data();
        }

        #[test]
        fn no_ancient_mats_non_ancient_weapon_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::OceanicComponents,
                MaterialName::OceanicComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Weapon;
            let is_ancient_gizmo = false;
            let expected = vec![
                PerkValues{ base: 97, perk: PerkName::Precise,      rolls: smallvec![8, 8, 32], ..Default::default() },
                PerkValues{ base: 90, perk: PerkName::Invigorating, rolls: smallvec![8, 8    ], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Cautious,     rolls: smallvec![44      ], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Blunted,      rolls: smallvec![45      ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Equilibrium,  rolls: smallvec![33      ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Flanking,     rolls: smallvec![32      ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn no_ancient_mats_non_ancient_armour_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::OceanicComponents,
                MaterialName::OceanicComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Armour;
            let is_ancient_gizmo = false;
            let expected = vec![
                PerkValues{ base: 78, perk: PerkName::Devoted,      rolls: smallvec![9, 9], ..Default::default() },
                PerkValues{ base: 90, perk: PerkName::Invigorating, rolls: smallvec![8, 8], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Cautious,     rolls: smallvec![44  ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn no_ancient_mats_non_ancient_tool_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::OceanicComponents,
                MaterialName::OceanicComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Tool;
            let is_ancient_gizmo = false;
            let expected = vec![
                PerkValues{ base: 50, perk: PerkName::Charitable, rolls: smallvec![28, 28], ..Default::default() },
                PerkValues{ base: 50, perk: PerkName::Polishing,  rolls: smallvec![28, 28], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Cautious,   rolls: smallvec![44    ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Honed,      rolls: smallvec![32    ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn ancient_mats_non_ancient_weapon_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::HistoricComponents,
                MaterialName::HistoricComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Weapon;
            let is_ancient_gizmo = false;
            let expected = vec![
                PerkValues{ base: 97, perk: PerkName::Precise,     rolls: smallvec![8, 8, 32], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Cautious,    rolls: smallvec![44      ], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Blunted,     rolls: smallvec![45      ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Equilibrium, rolls: smallvec![33      ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Flanking,    rolls: smallvec![32      ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn ancient_mats_non_ancient_armour_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::HistoricComponents,
                MaterialName::HistoricComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Armour;
            let is_ancient_gizmo = false;
            let expected = vec![
                PerkValues{ base: 78, perk: PerkName::Devoted,  rolls: smallvec![9, 9], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Cautious, rolls: smallvec![44  ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn ancient_mats_non_ancient_tool_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::HistoricComponents,
                MaterialName::HistoricComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Tool;
            let is_ancient_gizmo = false;
            let expected = vec![
                PerkValues{ base: 50, perk: PerkName::Charitable, rolls: smallvec![28, 28], ..Default::default() },
                PerkValues{ base: 12, perk: PerkName::Cautious,   rolls: smallvec![44    ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Honed,      rolls: smallvec![32    ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn ancient_mats_ancient_weapon_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::HistoricComponents,
                MaterialName::HistoricComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Weapon;
            let is_ancient_gizmo = true;
            let expected = vec![
                PerkValues{ base: 99, perk: PerkName::Precise,     rolls: smallvec![6, 6, 33, 33, 25], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Genocidal,   rolls: smallvec![33, 33          ], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Ultimatums,  rolls: smallvec![33, 33          ], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Looting,     rolls: smallvec![33, 33          ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Cautious,    rolls: smallvec![35              ], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Blunted,     rolls: smallvec![36              ], ..Default::default() },
                PerkValues{ base: 7,  perk: PerkName::Equilibrium, rolls: smallvec![26              ], ..Default::default() },
                PerkValues{ base: 7,  perk: PerkName::Flanking,    rolls: smallvec![25              ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }


        #[test]
        fn ancient_mats_ancient_armour_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::HistoricComponents,
                MaterialName::HistoricComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Armour;
            let is_ancient_gizmo = true;
            let expected = vec![
                PerkValues{ base: 62, perk: PerkName::Devoted,    rolls: smallvec![7, 7  ], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Genocidal,  rolls: smallvec![33, 33], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Ultimatums, rolls: smallvec![33, 33], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Looting,    rolls: smallvec![33, 33], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::Turtling,   rolls: smallvec![33, 33], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Cautious,   rolls: smallvec![35    ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }


        #[test]
        fn ancient_mats_ancient_tool_gizmo() {
            let input_materials = vec![
                MaterialName::ArmadylComponents,
                MaterialName::ArmadylComponents,
                MaterialName::HistoricComponents,
                MaterialName::HistoricComponents,
                MaterialName::PreciseComponents,
            ];
            let gizmo_type = GizmoType::Tool;
            let is_ancient_gizmo = true;
            let expected = vec![
                PerkValues{ base: 40, perk: PerkName::Charitable, rolls: smallvec![22, 22], ..Default::default() },
                PerkValues{ base: 22, perk: PerkName::ImpSouled,  rolls: smallvec![33, 33], ..Default::default() },
                PerkValues{ base: 9,  perk: PerkName::Cautious,   rolls: smallvec![35    ], ..Default::default() },
                PerkValues{ base: 7,  perk: PerkName::Honed,      rolls: smallvec![25    ], ..Default::default() },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_perk_values_eq(&actual, &expected);
        }
    }

    mod calc_perk_rank_probabilities_tests {
        use std::collections::HashMap;
        use super::*;

        lazy_static!{
            static ref DATA: Data = Data {
                comps: HashMap::new(),
                perks: HashMap::from([
                    (PerkName::Precise, PerkRanksData {
                        doubleslot: true,
                        ranks: vec![
                            PerkRankValues { perk: PerkName::Precise, doubleslot: true, rank: 0, threshold: 0, ancient_only: false, ..Default::default() },
                            PerkRankValues { perk: PerkName::Precise, doubleslot: true, rank: 1, threshold: 10, ancient_only: false, ..Default::default() },
                            PerkRankValues { perk: PerkName::Precise, doubleslot: true, rank: 2, threshold: 100, ancient_only: false, ..Default::default() },
                            PerkRankValues { perk: PerkName::Precise, doubleslot: true, rank: 3, threshold: 150, ancient_only: true, ..Default::default() },
                        ]
                    }),
                    (PerkName::Biting, PerkRanksData {
                        doubleslot: false,
                        ranks: vec![
                            PerkRankValues { perk: PerkName::Biting, doubleslot: false, rank: 0, threshold: 0, ancient_only: false, ..Default::default() },
                            PerkRankValues { perk: PerkName::Biting, doubleslot: false, rank: 1, threshold: 50, ancient_only: false, ..Default::default() },
                            PerkRankValues { perk: PerkName::Biting, doubleslot: false, rank: 2, threshold: 80, ancient_only: false, ..Default::default() },
                            PerkRankValues { perk: PerkName::Biting, doubleslot: false, rank: 3, threshold: 200, ancient_only: true, ..Default::default() },
                            PerkRankValues { perk: PerkName::Biting, doubleslot: false, rank: 4, threshold: 250, ancient_only: true, ..Default::default() },
                        ]
                    }),
                ])
            };
        }

        #[test]
        fn all_ranks_possible_not_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![32, 32, 64], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    rolls: smallvec![32, 32, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.87188720703125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.12811279296875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.0 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, false);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn all_ranks_possible_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![128, 128], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    rolls: smallvec![128, 128],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.24993896484375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.34295654296875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.4071044921875 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, true);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn two_perks_all_ranks_possible_non_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![32, 32, 64], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![32, 32, 64], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    rolls: smallvec![32, 32, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.87188720703125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.12811279296875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.0 },
                    ]
                },
                PerkValues {
                    base: 50,
                    doubleslot: false,
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    rolls: smallvec![32, 32, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[1], probability: 0.07568359375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[2], probability: 0.92431640625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[3], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[4], probability: 0.0 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, false);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn two_perks_all_ranks_possible_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![128, 128, 64], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![32, 128, 128], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    rolls: smallvec![64, 128, 128],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.11663818359375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.25565338134765625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.62770843505859375 },
                    ]
                },
                PerkValues {
                    base: 50,
                    doubleslot: false,
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 4,
                    rolls: smallvec![32, 128, 128],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[1], probability: 0.00946044921875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[2], probability: 0.541595458984375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[3], probability: 0.292510986328125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[4], probability: 0.15643310546875 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, true);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn not_all_ranks_possible_non_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 5, rolls: smallvec![16, 16, 32], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 5,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 1,
                    rolls: smallvec![16, 16, 32],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.0042724609375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.9957275390625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.0 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, false);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn not_all_ranks_possible_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Biting, base: 5, rolls: smallvec![32, 64, 64, 64], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 5,
                    doubleslot: false,
                    perk: PerkName::Biting,
                    i_first: 0,
                    i_last: 3,
                    rolls: smallvec![32, 64, 64, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[0], probability: 0.02297878265380859375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[1], probability: 0.12725317478179931641 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[2], probability: 0.84693670272827148438 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[3], probability: 0.00283133983612060547 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[4], probability: 0.0 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, true);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn two_perks_not_all_ranks_possible_non_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 5, rolls: smallvec![32, 32, 64, 16, 16], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 5, rolls: smallvec![32, 32, 64, 16, 16], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 5,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    rolls: smallvec![16, 16, 32, 32, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.00000751018524169921875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.74765408039093017578125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.252338409423828125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.0 },
                    ]
                },
                PerkValues {
                    base: 5,
                    doubleslot: false,
                    perk: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    rolls: smallvec![16, 16, 32, 32, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[0], probability: 0.084997653961181640625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[1], probability: 0.369161128997802734375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[2], probability: 0.545841217041015625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[3], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[4], probability: 0.0 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, false);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn two_perks_not_all_ranks_possible_ancient_gizmo() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Precise, base: 5, rolls: smallvec![32, 64, 16], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 5, rolls: smallvec![32, 64, 64, 64], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 5,
                    doubleslot: true,
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    rolls: smallvec![16, 32, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[0], probability: 0.001068115234375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[1], probability: 0.978179931640625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[2], probability: 0.020751953125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Precise].ranks[3], probability: 0.0 },
                    ]
                },
                PerkValues {
                    base: 5,
                    doubleslot: false,
                    perk: PerkName::Biting,
                    i_first: 0,
                    i_last: 3,
                    rolls: smallvec![32, 64, 64, 64],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[0], probability: 0.02297878265380859375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[1], probability: 0.12725317478179931641 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[2], probability: 0.84693670272827148438 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[3], probability: 0.00283133983612060547 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[4], probability: 0.0 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, true);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }

        #[test]
        fn high_base_value() {
            let mut perk_values_arr =  vec![
                PerkValues { perk: PerkName::Biting, base: 100, rolls: smallvec![250], ..Default::default() },
            ];
            let expected = vec![
                PerkValues {
                    base: 100,
                    doubleslot: false,
                    perk: PerkName::Biting,
                    i_first: 2,
                    i_last: 4,
                    rolls: smallvec![250],
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[1], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[2], probability: 0.4 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[3], probability: 0.2 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[&PerkName::Biting].ranks[4], probability: 0.4 },
                    ]
                }
            ];
            calc_perk_rank_probabilities(&*DATA, &mut perk_values_arr, true);
            assert_perk_values_eq(&perk_values_arr, &expected);
        }
    }

    mod can_generate_wanted_ranks_test {
        use super::*;
        use std::collections::HashMap;

        lazy_static!{
            static ref DATA: Data = Data {
                comps: HashMap::new(),
                perks: HashMap::from([
                    (PerkName::Empty, PerkRanksData {
                        doubleslot: false,
                        ranks: vec![
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                        ]
                    }),
                    (PerkName::Precise, PerkRanksData {
                        doubleslot: false,
                        ranks: vec![
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                            PerkRankValues { rank: 1, threshold: 10, ..Default::default() },
                            PerkRankValues { rank: 2, threshold: 100, ..Default::default() },
                            PerkRankValues { rank: 3, threshold: 150, ..Default::default() },
                        ]
                    }),
                    (PerkName::Biting, PerkRanksData {
                        doubleslot: false,
                        ranks: vec![
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                            PerkRankValues { rank: 1, threshold: 50, ..Default::default() },
                            PerkRankValues { rank: 2, threshold: 80, ..Default::default() },
                            PerkRankValues { rank: 3, threshold: 200, ..Default::default() },
                            PerkRankValues { rank: 4, threshold: 250, ..Default::default() },
                        ]
                    }),
                    (PerkName::Equilibrium, PerkRanksData {
                        doubleslot: false,
                        ranks: vec![
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                            PerkRankValues { rank: 1, threshold: 50, ..Default::default() },
                            PerkRankValues { rank: 2, threshold: 80, ..Default::default() },
                            PerkRankValues { rank: 3, threshold: 200, ..Default::default() },
                            PerkRankValues { rank: 4, threshold: 250, ..Default::default() },
                        ]
                    }),
                ])
            };
        }

        #[test]
        fn single_wanted_not_in_perk_values() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 50, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Equilibrium, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Empty, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn first_wanted_not_in_perk_values() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 50, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Equilibrium, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn second_wanted_not_in_perk_values() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 50, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Equilibrium, rank: 2, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn single_wanted_pv_below_threshold() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![20, 71], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Empty, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn first_wanted_pv_below_threshold() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![20, 71], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Biting, rank: 1, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn second_wanted_pv_below_threshold() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 10, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn single_wanted_pv_above_threshold() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 50, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 12, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                WantedPerk { perk: PerkName::Empty, ..Default::default() },
            );
            assert_eq!(true, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn both_wanted_pv_above_threshold() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 50, rolls: smallvec![20, 40], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
            );
            assert_eq!(true, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn first_wanted_pv_base_too_high() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 80, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 100, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }

        #[test]
        fn second_wanted_pv_base_too_high() {
            let perk_values_arr = vec![
                PerkValues { perk: PerkName::Precise, base: 160, rolls: smallvec![20, 20], ..Default::default() },
                PerkValues { perk: PerkName::Biting, base: 50, rolls: smallvec![20, 20], ..Default::default() },
            ];
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Biting, rank: 1, ..Default::default() },
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
            );
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, &wanted_gizmo))
        }
    }

    mod permutate_perk_ranks_test {
        use super::*;

        fn assert_rank_combination_eq(actual: &Vec<RankCombination>, expected: &Vec<RankCombination>) {
            assert!(actual.len() == expected.len(), "Actual and expected have different sizes (actual: {}, expected: {})", actual.len(), expected.len());

            for x in expected {
                assert!(actual.contains(&x), "Actual doesn't contain {:#?}", x);
            }
        }

        #[test]
        fn permutate_ranks() {
            let perk_list = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, perk: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, perk: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, perk: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, perk: PerkName::Precise, ..Default::default() }},
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 2,
                    i_last: 3,
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, perk: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, perk: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, perk: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, perk: PerkName::Biting, ..Default::default() }},
                    ],
                    ..Default::default()
                },
            ];
            let expected = vec![
                RankCombination {
                    ranks: smallvec![
                        perk_list[0].ranks[1].values,
                        perk_list[1].ranks[2].values,
                    ],
                    probability: 1.0/32.0
                },
                RankCombination {
                    ranks: smallvec![
                        perk_list[0].ranks[2].values,
                        perk_list[1].ranks[2].values,
                    ],
                    probability: 1.0/16.0
                },
                RankCombination {
                    ranks: smallvec![
                        perk_list[0].ranks[3].values,
                        perk_list[1].ranks[2].values,
                    ],
                    probability: 1.0/8.0
                },
                RankCombination {
                    ranks: smallvec![
                        perk_list[0].ranks[1].values,
                        perk_list[1].ranks[3].values,
                    ],
                    probability: 1.0/16.0
                },
                RankCombination {
                    ranks: smallvec![
                        perk_list[0].ranks[2].values,
                        perk_list[1].ranks[3].values,
                    ],
                    probability: 1.0/8.0
                },
                RankCombination {
                    ranks: smallvec![
                        perk_list[0].ranks[3].values,
                        perk_list[1].ranks[3].values,
                    ],
                    probability: 1.0/4.0
                },
            ];
            let actual = permutate_perk_ranks(&perk_list, None);
            assert_rank_combination_eq(&actual, &expected);
        }

        lazy_static!{
            static ref PERK_LIST: Vec<PerkValues> = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, perk: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, perk: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, perk: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, perk: PerkName::Precise, ..Default::default() }},
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 2,
                    i_last: 3,
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, perk: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, perk: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, perk: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, perk: PerkName::Biting, ..Default::default() }},
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Equilibrium,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, perk: PerkName::Equilibrium, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 1, perk: PerkName::Equilibrium, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 2, perk: PerkName::Equilibrium, ..Default::default() }},
                    ],
                    ..Default::default()
                },
            ];
        }

        #[test]
        fn permutate_ranks_one_wanted() {
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Empty, ..Default::default() },
            );
            let expected = vec![
                RankCombination {
                    ranks: smallvec![
                        PERK_LIST[0].ranks[2].values,
                        PERK_LIST[1].ranks[2].values,
                        PERK_LIST[2].ranks[1].values,
                    ],
                    probability: 1.0/64.0
                },
                RankCombination {
                    ranks: smallvec![
                        PERK_LIST[0].ranks[2].values,
                        PERK_LIST[1].ranks[3].values,
                        PERK_LIST[2].ranks[1].values,
                    ],
                    probability: 1.0/32.0
                },
                RankCombination {
                    ranks: smallvec![
                        PERK_LIST[0].ranks[2].values,
                        PERK_LIST[1].ranks[2].values,
                        PERK_LIST[2].ranks[2].values,
                    ],
                    probability: 1.0/32.0
                },
                RankCombination {
                    ranks: smallvec![
                        PERK_LIST[0].ranks[2].values,
                        PERK_LIST[1].ranks[3].values,
                        PERK_LIST[2].ranks[2].values,
                    ],
                    probability: 1.0/16.0
                },
            ];
            let actual = permutate_perk_ranks(&*PERK_LIST, Some(&wanted_gizmo));
            assert_rank_combination_eq(&actual, &expected);
        }

        #[test]
        fn permutate_ranks_two_wanted() {
            let wanted_gizmo = WantedGizmo (
                WantedPerk { perk: PerkName::Precise, rank: 2, ..Default::default() },
                WantedPerk { perk: PerkName::Equilibrium, rank: 1, ..Default::default() },
            );
            let expected = vec![
                RankCombination {
                    ranks: smallvec![
                        PERK_LIST[0].ranks[2].values,
                        PERK_LIST[1].ranks[2].values,
                        PERK_LIST[2].ranks[1].values,
                    ],
                    probability: 1.0/64.0
                },
                RankCombination {
                    ranks: smallvec![
                        PERK_LIST[0].ranks[2].values,
                        PERK_LIST[1].ranks[3].values,
                        PERK_LIST[2].ranks[1].values,
                    ],
                    probability: 1.0/32.0
                },
            ];
            let actual = permutate_perk_ranks(&*PERK_LIST, Some(&wanted_gizmo));
            assert_rank_combination_eq(&actual, &expected);
        }
    }

    mod get_empty_gizmo_chance_tests {
        use super::*;
        use approx::assert_relative_eq;

        lazy_static!{
            static ref BUDGET: Budget = Budget {
                dist: vec![
                    0.0, 0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.11, 0.12, 0.13, 0.14, 0.15, 0.16,
                    0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.28, 0.29, 0.3, 0.31, 0.32, 0.33,
                    0.34, 0.35, 0.36, 0.37, 0.38, 0.39, 0.4, 0.41, 0.42, 0.43, 0.44, 0.45, 0.46, 0.47, 0.48, 0.49, 0.5,
                    0.51, 0.52, 0.53, 0.54, 0.55, 0.56, 0.57, 0.58, 0.59, 0.6, 0.61, 0.62, 0.63, 0.64, 0.65, 0.66, 0.67,
                    0.68, 0.69, 0.7, 0.71, 0.72, 0.73, 0.74, 0.75, 0.76, 0.77, 0.78, 0.79, 0.8, 0.81, 0.82, 0.83, 0.84,
                    0.85, 0.86, 0.87, 0.88, 0.89, 0.9, 0.91, 0.92, 0.93, 0.94, 0.95, 0.96, 0.97, 0.98, 0.99, 1.0,
                ],
                level: 25,
                range: Range { min: 25, max: 100 }
            };
        }

        #[test]
        fn all_have_zero_rank_one_has_cost_below_min_range() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 40, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.434375;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn all_have_zero_rank_all_have_one_cost_below_min_range() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.390625;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn all_have_zero_rank_all_cost_below_min_range() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 20, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.015625;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn not_all_have_zero_rank_all_cost_below_min_range() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 20, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.0;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn zero_and_above_plus_no_zero_and_below() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 150, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 20, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.0;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn zero_and_above_plus_no_zero_and_above() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.09375;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn no_zero_and_below_plus_no_zero_and_above() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.0;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }

        #[test]
        fn no_zero_and_above_plus_no_zero_and_above() {
            let pv = vec![
                PerkValues {
                    perk: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 1, cost: 100, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Precise, rank: 2, cost: 150, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
                PerkValues {
                    perk: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: smallvec![
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { perk: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
                    ],
                    ..Default::default()
                },
            ];
            let expected = 0.65625;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }
    }
}