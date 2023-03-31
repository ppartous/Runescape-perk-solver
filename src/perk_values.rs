use itertools::Itertools;
use smallvec::{SmallVec, smallvec};
use strum::EnumCount;
use crate::{prelude::*, dice, utils};
use std::sync::Arc;

/// Calculate the base and roll values for each possible perk based on the input materials and their order.
pub fn get_perk_values(data: &Data, input_materials: &Vec<MaterialName>, gizmo_type: GizmoType,
    is_ancient_gizmo: bool) -> PartialPerkValuesVec {

    let mut perk_values = smallvec![];
    let mut name_index_map = StackMap::<PerkName, Option<usize>, {PerkName::COUNT}>::new();

    for mat in input_materials {
        let mat_data = &data.comps[*mat][gizmo_type];
        let is_ancient_mat = data.comps[*mat].ancient_only;

        if is_ancient_mat && !is_ancient_gizmo {
            continue;
        }

        for component_values in mat_data {
            let mut perk_roll = component_values.roll as u64;
            let mut perk_base = component_values.base as u64;

            if is_ancient_gizmo && !is_ancient_mat {
                perk_roll = (perk_roll * 8) / 10;
                perk_base = (perk_base * 8) / 10;
            }

            if let Some(index) = name_index_map.get(component_values.perk) {
                let mut values: &mut PartialPerkValues = unsafe { perk_values.get_unchecked_mut(*index) };
                values.base += perk_base as u16;
                values.rolls.push(perk_roll as u8);
            } else {
                perk_values.push(PartialPerkValues {
                    name: component_values.perk,
                    base: perk_base as u16,
                    rolls: StackVec::new(&[perk_roll as u8])
                });
                *name_index_map.get_mut(component_values.perk) = Some(perk_values.len() - 1);
            }
        }
    }

    perk_values
}

pub fn calc_perk_rank_probabilities(data: &Data, partial_values_arr: &[PartialPerkValues], is_ancient_gizmo: bool) -> PerkValuesVec {
    let mut perk_values_arr = SmallVec::with_capacity(partial_values_arr.len());

    for partial_values in partial_values_arr.iter() {
        let perk_data = &data.perks[partial_values.name];

        let mut perk_values = PerkValues {
            name: partial_values.name,
            base: partial_values.base,
            rolls: partial_values.rolls,
            i_first: 0,
            i_last: perk_data.ranks.len() - 1,
            doubleslot: perk_data.doubleslot,
            ..Default::default()
        };

        for x in perk_data.ranks.iter() {
            perk_values.ranks.push(PerkRankValuesProbabilityContainer { values: *x, probability: 0.0 });
        }

        perk_values.rolls.sort();
        let mut roll_dist = Arc::new(Vec::new());

        let mut iter = perk_values.rolls.iter().peekable();
        let mut count = 1;
        while let Some(x) = iter.next() {
            while let Some(next) = iter.peek() {
                if *next != x { break; }
                count += 1;
                iter.next();
            }

            if !roll_dist.is_empty() {
                roll_dist = Arc::new(utils::convolve(&roll_dist, &dice::get_distribution(*x as usize, count)));
            } else {
                roll_dist = dice::get_distribution(*x as usize, count);
            }
            count = 1;
        }

        for i in 0..(perk_values.ranks.len()) {
            let mut next_threshold = roll_dist.len() as i64 - 1;

            if i + 1 < perk_values.ranks.len() {
                let next_container = unsafe{ perk_values.ranks.get_unchecked(i + 1) };
                if !next_container.values.ancient_only || is_ancient_gizmo {
                    // -1 because the range we need ends right before the threshold value of the next perk
                    next_threshold = next_threshold.min(next_container.values.threshold as i64 - perk_values.base as i64 - 1);
                }
            }

            let mut container = unsafe{ perk_values.ranks.get_unchecked_mut(i) };
            let range_start = i64::max(container.values.threshold as i64 - perk_values.base as i64, 0) as usize;
            if (is_ancient_gizmo || !container.values.ancient_only) && (range_start as i64) <= next_threshold && next_threshold >= 0 {
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

        perk_values_arr.push(perk_values);
    }

    perk_values_arr
}

/// Quick check if it is even possible to generate the wanted perk rank. This won't catch all impossible material orders.
pub fn can_generate_wanted_ranks(data: &Data, perk_values_arr: &PartialPerkValuesVec, wanted_gizmo: Gizmo) -> bool {
    let wanted_rank1 = wanted_gizmo.perks.0.rank as usize;
    let wanted_rank2 = wanted_gizmo.perks.1.rank as usize;

    let perk1_ranks = &data.perks[wanted_gizmo.perks.0.name].ranks;
    let perk2_ranks = &data.perks[wanted_gizmo.perks.1.name].ranks;

    let perk1_threshold = perk1_ranks[wanted_rank1].threshold as usize;
    let perk1_next_threshold = if wanted_rank1 + 1 < perk1_ranks.len() { perk1_ranks[wanted_rank1 + 1].threshold as usize } else { usize::MAX };

    let perk2_threshold = perk2_ranks[wanted_rank2].threshold as usize;
    let perk2_next_threshold = if wanted_rank2 + 1 < perk2_ranks.len() { perk2_ranks[wanted_rank2 + 1].threshold as usize } else { usize::MAX };

    let mut perk1_base = None;
    let mut perk1_max_roll = None;
    let mut perk2_base = None;
    let mut perk2_max_roll = None;

    for perk_value in perk_values_arr {
        if perk_value.name == wanted_gizmo.perks.0.name {
            perk1_base = Some(perk_value.base as usize);
            perk1_max_roll = Some(perk_value.rolls.iter().map(|x| *x as usize).sum::<usize>() - perk_value.rolls.len());
        }
        if perk_value.name == wanted_gizmo.perks.1.name {
            perk2_base = Some(perk_value.base as usize);
            perk2_max_roll = Some(perk_value.rolls.iter().map(|x| *x as usize).sum::<usize>() - perk_value.rolls.len());
        }
        if perk1_base.is_some() && perk2_base.is_some() {
            break;
        }
    }

    if perk1_base.is_none() || (wanted_gizmo.perks.1.name != PerkName::Empty && perk2_base.is_none()) {
        return false;
    }
    unsafe {
        if !(perk1_base.unwrap_unchecked() + perk1_max_roll.unwrap_unchecked() >= perk1_threshold && perk1_base.unwrap_unchecked() < perk1_next_threshold) {
            return false;
        }
        if wanted_gizmo.perks.1.name != PerkName::Empty
        && !(perk2_base.unwrap_unchecked() + perk2_max_roll.unwrap_unchecked() >= perk2_threshold && perk2_base.unwrap_unchecked() < perk2_next_threshold) {
            return false;
        }
    }

    true
}

pub fn permutate_perk_ranks<'a>(perk_list: &'a PerkValuesVec, wanted_gizmo: Option<Gizmo>) -> Vec<RankCombination> {
    let mut combinations = Vec::with_capacity(perk_list.len() * 10);

    let func = |x: &'a PerkValues| {
        let mut i_first = x.i_first;
        let mut i_last = x.i_last;
        match wanted_gizmo {
            Some(gizmo) if gizmo.perks.0.name == x.name => {
                i_first = gizmo.perks.0.rank as usize;
                i_last = i_first;
            },
            Some(gizmo) if gizmo.perks.1.name == x.name => {
                i_first = gizmo.perks.1.rank as usize;
                i_last = i_first;
            },
            _ => ()
        }
        x.ranks.iter().take(i_last + 1).skip(i_first)
    };

    for pv_combination in perk_list.iter().map(func).multi_cartesian_product() {
        let mut probability = 1.0;
        let mut ranks = SmallVec::<[PerkRankValues; 12]>::new();

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
pub fn get_empty_gizmo_chance(budget: &Budget, perk_values_arr: &[PerkValues]) -> f64 {
    let mut p_empty = 1.0; // Total empty gizmo chance
    let mut p_empty_per_perk = StackMap::<PerkName, f64, {PerkName::COUNT}>::new();
    let mut ranks: SmallVec<[PRVPC; 30]> = smallvec![]; // vec of non zero ranks with a cost higher than the invention level

    // Chance to have a combination of perk ranks that can produce an empty gizmo (perks with a cost higher than
    // the invention level or of rank 0)
    let mut p_empty_combo = 1.0;
    for pv in perk_values_arr.iter() {
        let pv_ranks = &pv.ranks;

        p_empty *= pv_ranks[0].probability;

        let mut psum = pv_ranks[0].probability;
        for rank in pv_ranks.iter().take(pv.i_last + 1).skip(pv.i_first).rev() {
            if rank.values.cost < budget.range.min {
                break;
            }

            psum += rank.probability;
            ranks.push(*rank);
        }

        *p_empty_per_perk.get_mut(pv.name) = psum;
        p_empty_combo *= psum;
    }

    if p_empty_combo == 0.0 {
        return 0.0;
    } else if ranks.is_empty() {
        return p_empty;
    }

    ranks.sort_unstable_by(|x, y| x.values.cost.cmp(&y.values.cost));

    for rank in ranks.iter() {
        // Adjusted empty combo chance to a specific rank of a perk
        let mut p_empty_rank = p_empty_combo * rank.probability / p_empty_per_perk.get(rank.values.name);
        unsafe {
            // Multiply with chance that our inventbudget is bellow the rank cost
            p_empty_rank *= budget.dist.get_unchecked(u16::min(rank.values.cost, budget.range.max) as usize);
        }
        p_empty += p_empty_rank;
        // Remove this rank from 'p_empty_combo'
        p_empty_combo *= (p_empty_per_perk.get(rank.values.name) - rank.probability) / p_empty_per_perk.get(rank.values.name);
        // Remove this rank from the combined empty combo chance of a certain perk
        *p_empty_per_perk.get_mut(rank.values.name) -= rank.probability;

        if p_empty_combo == 0.0 {
            break;
        }
    }

    p_empty
}

pub fn contains_conflict_ranks(data: &Data, perk_values_arr: &[PerkValues], wanted_gizmo: Gizmo) -> bool {
    let p1_cost = data.perks[wanted_gizmo.perks.0.name].ranks[wanted_gizmo.perks.0.rank as usize].cost;
    let p2_cost = data.perks[wanted_gizmo.perks.1.name].ranks[wanted_gizmo.perks.1.rank as usize].cost;

    if p1_cost == p2_cost {
        return true;
    }

    for perk_values in perk_values_arr.iter() {
        if perk_values.name == wanted_gizmo.perks.0.name || perk_values.name == wanted_gizmo.perks.1.name {
            continue;
        }

        for rank in perk_values.ranks.iter().take(perk_values.i_last + 1).skip(perk_values.i_first) {
            if rank.values.cost == p1_cost || rank.values.cost == p2_cost {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use once_cell::sync::Lazy;
    use crate::utils::{check_len, check_index, check_index_relative};

    fn assert_partial_perk_values_eq(actual: &PartialPerkValuesVec, expected: &PartialPerkValuesVec) {
        check_len(actual, expected);

        for (i, (acc, exp)) in actual.iter().zip(expected).enumerate() {
            check_index(acc.name, exp.name, i, "name", actual, expected);
            check_index(acc.base, exp.base, i, "base", actual, expected);

            for (x, y) in acc.rolls.iter().zip_eq(&exp.rolls) {
                check_index(*x, *y, i, "rolls", actual, expected);
            }
        }
    }

    fn assert_perk_values_eq(actual: &PerkValuesVec, expected: &PerkValuesVec) {
        check_len(actual, expected);

        for (i, (acc, exp)) in actual.iter().zip(expected).enumerate() {
            check_index(acc.base, exp.base, i, "base", actual, expected);
            check_index(acc.name, exp.name, i, "name", actual, expected);
            check_index(acc.doubleslot, exp.doubleslot, i, "doubleslot", actual, expected);
            check_index(acc.i_first, exp.i_first, i, "i_first", actual, expected);
            check_index(acc.i_last, exp.i_last, i, "i_last", actual, expected);

            for (x, y) in acc.rolls.iter().zip_eq(&exp.rolls) {
                check_index(*x, *y, i, "rolls", actual, expected);
            }

            for (x, y) in acc.ranks.iter().zip_eq(&exp.ranks) {
                check_index(x.values, y.values, i, "ranks.values", actual, expected);
                check_index_relative(x.probability, y.probability, 8.0, i, "ranks.probability", actual, expected);
            }
        }
    }

    mod get_perk_values_tests {
        use super::*;
        static DATA: Lazy<Data> = Lazy::new(|| Data::load());

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
            let expected = smallvec![
                PartialPerkValues{ base: 97, name: PerkName::Precise,      rolls: StackVec::new(&[8, 8, 32]) },
                PartialPerkValues{ base: 90, name: PerkName::Invigorating, rolls: StackVec::new(&[8, 8    ]) },
                PartialPerkValues{ base: 12, name: PerkName::Cautious,     rolls: StackVec::new(&[44      ]) },
                PartialPerkValues{ base: 12, name: PerkName::Blunted,      rolls: StackVec::new(&[45      ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Equilibrium,  rolls: StackVec::new(&[33      ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Flanking,     rolls: StackVec::new(&[32      ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 78, name: PerkName::Devoted,      rolls: StackVec::new(&[9, 9]) },
                PartialPerkValues{ base: 90, name: PerkName::Invigorating, rolls: StackVec::new(&[8, 8]) },
                PartialPerkValues{ base: 12, name: PerkName::Cautious,     rolls: StackVec::new(&[44  ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 50, name: PerkName::Charitable, rolls: StackVec::new(&[28, 28]) },
                PartialPerkValues{ base: 50, name: PerkName::Polishing,  rolls: StackVec::new(&[28, 28]) },
                PartialPerkValues{ base: 12, name: PerkName::Cautious,   rolls: StackVec::new(&[44    ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Honed,      rolls: StackVec::new(&[32    ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 97, name: PerkName::Precise,     rolls: StackVec::new(&[8, 8, 32]) },
                PartialPerkValues{ base: 12, name: PerkName::Cautious,    rolls: StackVec::new(&[44      ]) },
                PartialPerkValues{ base: 12, name: PerkName::Blunted,     rolls: StackVec::new(&[45      ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Equilibrium, rolls: StackVec::new(&[33      ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Flanking,    rolls: StackVec::new(&[32      ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 78, name: PerkName::Devoted,  rolls: StackVec::new(&[9, 9]) },
                PartialPerkValues{ base: 12, name: PerkName::Cautious, rolls: StackVec::new(&[44  ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 50, name: PerkName::Charitable, rolls: StackVec::new(&[28, 28]) },
                PartialPerkValues{ base: 12, name: PerkName::Cautious,   rolls: StackVec::new(&[44    ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Honed,      rolls: StackVec::new(&[32    ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 99, name: PerkName::Precise,     rolls: StackVec::new(&[6, 6, 33, 33, 25]) },
                PartialPerkValues{ base: 22, name: PerkName::Genocidal,   rolls: StackVec::new(&[33, 33          ]) },
                PartialPerkValues{ base: 22, name: PerkName::Ultimatums,  rolls: StackVec::new(&[33, 33          ]) },
                PartialPerkValues{ base: 22, name: PerkName::Looting,     rolls: StackVec::new(&[33, 33          ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Cautious,    rolls: StackVec::new(&[35              ]) },
                PartialPerkValues{ base: 9,  name: PerkName::Blunted,     rolls: StackVec::new(&[36              ]) },
                PartialPerkValues{ base: 7,  name: PerkName::Equilibrium, rolls: StackVec::new(&[26              ]) },
                PartialPerkValues{ base: 7,  name: PerkName::Flanking,    rolls: StackVec::new(&[25              ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 62, name: PerkName::Devoted,    rolls: StackVec::new(&[7, 7  ]) },
                PartialPerkValues{ base: 22, name: PerkName::Genocidal,  rolls: StackVec::new(&[33, 33]) },
                PartialPerkValues{ base: 22, name: PerkName::Ultimatums, rolls: StackVec::new(&[33, 33]) },
                PartialPerkValues{ base: 22, name: PerkName::Looting,    rolls: StackVec::new(&[33, 33]) },
                PartialPerkValues{ base: 22, name: PerkName::Turtling,   rolls: StackVec::new(&[33, 33]) },
                PartialPerkValues{ base: 9,  name: PerkName::Cautious,   rolls: StackVec::new(&[35    ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
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
            let expected = smallvec![
                PartialPerkValues{ base: 40, name: PerkName::Charitable, rolls: StackVec::new(&[22, 22]) },
                PartialPerkValues{ base: 22, name: PerkName::ImpSouled,  rolls: StackVec::new(&[33, 33]) },
                PartialPerkValues{ base: 9,  name: PerkName::Cautious,   rolls: StackVec::new(&[35    ]) },
                PartialPerkValues{ base: 7,  name: PerkName::Honed,      rolls: StackVec::new(&[25    ]) },
            ];
            let actual = get_perk_values(&*DATA, &input_materials, gizmo_type, is_ancient_gizmo);
            assert_partial_perk_values_eq(&actual, &expected);
        }
    }

    mod calc_perk_rank_probabilities_tests {
        use crate::{stack_map::StackMap, stack_vec::StackVec};
        use super::*;

        static DATA: Lazy<Data> = Lazy::new(|| {
            Data {
                comps: StackMap::new(),
                perks: {
                    let mut map = StackMap::new();
                    map.insert(PerkName::Precise, PerkRanksData {
                        doubleslot: true,
                        ranks: StackVec::new(&[
                            PerkRankValues { name: PerkName::Precise, doubleslot: true, rank: 0, threshold: 0, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Precise, doubleslot: true, rank: 1, threshold: 10, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Precise, doubleslot: true, rank: 2, threshold: 100, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Precise, doubleslot: true, rank: 3, threshold: 150, ancient_only: true, ..Default::default() },
                        ])
                    });
                    map.insert(PerkName::Biting, PerkRanksData {
                        doubleslot: false,
                        ranks: StackVec::new(&[
                            PerkRankValues { name: PerkName::Biting, doubleslot: false, rank: 0, threshold: 0, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Biting, doubleslot: false, rank: 1, threshold: 50, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Biting, doubleslot: false, rank: 2, threshold: 80, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Biting, doubleslot: false, rank: 3, threshold: 200, ancient_only: true, ..Default::default() },
                            PerkRankValues { name: PerkName::Biting, doubleslot: false, rank: 4, threshold: 250, ancient_only: true, ..Default::default() },
                        ])
                    });
                    map.insert(PerkName::Equilibrium, PerkRanksData {
                        doubleslot: false,
                        ranks: StackVec::new(&[
                            PerkRankValues { name: PerkName::Equilibrium, doubleslot: false, rank: 0, threshold: 0, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Equilibrium, doubleslot: false, rank: 1, threshold: 49, ancient_only: false, ..Default::default() },
                            PerkRankValues { name: PerkName::Equilibrium, doubleslot: false, rank: 2, threshold: 80, ancient_only: false, ..Default::default() },
                        ])
                    });
                    map
                }
            }
        });

        #[test]
        fn all_ranks_possible_not_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[32, 32, 64]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    rolls: StackVec::new(&[32, 32, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.87188720703125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.12811279296875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, false);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn all_ranks_possible_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[128, 128]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    rolls: StackVec::new(&[128, 128]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.24993896484375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.34295654296875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.4071044921875 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, true);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn two_perks_all_ranks_possible_non_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[32, 32, 64]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[32, 32, 64]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    rolls: StackVec::new(&[32, 32, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.87188720703125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.12811279296875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.0 },
                    ])
                },
                PerkValues {
                    base: 50,
                    doubleslot: false,
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    rolls: StackVec::new(&[32, 32, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[1], probability: 0.07568359375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[2], probability: 0.92431640625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[3], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[4], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, false);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn two_perks_all_ranks_possible_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[128, 128, 64]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[32, 128, 128]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 10,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    rolls: StackVec::new(&[64, 128, 128]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.11663818359375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.25565338134765625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.62770843505859375 },
                    ])
                },
                PerkValues {
                    base: 50,
                    doubleslot: false,
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 4,
                    rolls: StackVec::new(&[32, 128, 128]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[1], probability: 0.00946044921875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[2], probability: 0.541595458984375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[3], probability: 0.292510986328125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[4], probability: 0.15643310546875 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, true);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn not_all_ranks_possible_non_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 5, rolls: StackVec::new(&[16, 16, 32]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 5,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 1,
                    rolls: StackVec::new(&[16, 16, 32]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.0042724609375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.9957275390625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, false);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn not_all_ranks_possible_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Biting, base: 5, rolls: StackVec::new(&[32, 64, 64, 64]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 5,
                    doubleslot: false,
                    name: PerkName::Biting,
                    i_first: 0,
                    i_last: 3,
                    rolls: StackVec::new(&[32, 64, 64, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[0], probability: 0.02297878265380859375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[1], probability: 0.12725317478179931641 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[2], probability: 0.84693670272827148438 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[3], probability: 0.00283133983612060547 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[4], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, true);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn two_perks_not_all_ranks_possible_non_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 5, rolls: StackVec::new(&[32, 32, 64, 16, 16]) },
                PartialPerkValues { name: PerkName::Biting, base: 5, rolls: StackVec::new(&[32, 32, 64, 16, 16]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 5,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    rolls: StackVec::new(&[16, 16, 32, 32, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.00000751018524169921875 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.74765408039093017578125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.252338409423828125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.0 },
                    ])
                },
                PerkValues {
                    base: 5,
                    doubleslot: false,
                    name: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    rolls: StackVec::new(&[16, 16, 32, 32, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[0], probability: 0.084997653961181640625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[1], probability: 0.369161128997802734375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[2], probability: 0.545841217041015625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[3], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[4], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, false);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn two_perks_not_all_ranks_possible_ancient_gizmo() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Precise, base: 5, rolls: StackVec::new(&[32, 64, 16]) },
                PartialPerkValues { name: PerkName::Biting, base: 5, rolls: StackVec::new(&[32, 64, 64, 64]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 5,
                    doubleslot: true,
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    rolls: StackVec::new(&[16, 32, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[0], probability: 0.001068115234375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[1], probability: 0.978179931640625 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[2], probability: 0.020751953125 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Precise].ranks[3], probability: 0.0 },
                    ])
                },
                PerkValues {
                    base: 5,
                    doubleslot: false,
                    name: PerkName::Biting,
                    i_first: 0,
                    i_last: 3,
                    rolls: StackVec::new(&[32, 64, 64, 64]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[0], probability: 0.02297878265380859375 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[1], probability: 0.12725317478179931641 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[2], probability: 0.84693670272827148438 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[3], probability: 0.00283133983612060547 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[4], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, true);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn high_base_value() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Biting, base: 100, rolls: StackVec::new(&[250]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 100,
                    doubleslot: false,
                    name: PerkName::Biting,
                    i_first: 2,
                    i_last: 4,
                    rolls: StackVec::new(&[250]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[0], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[1], probability: 0.0 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[2], probability: 0.4 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[3], probability: 0.2 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Biting].ranks[4], probability: 0.4 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, true);
            assert_perk_values_eq(&actual, &expected);
        }

        #[test]
        fn threshold_equal_to_max_roll_plus_base() {
            let partial_perk_values = vec![
                PartialPerkValues { name: PerkName::Equilibrium, base: 10, rolls: StackVec::new(&[40]) },
            ];
            let expected = smallvec![
                PerkValues {
                    base: 10,
                    doubleslot: false,
                    name: PerkName::Equilibrium,
                    i_first: 0,
                    i_last: 1,
                    rolls: StackVec::new(&[40]),
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Equilibrium].ranks[0], probability: 0.975 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Equilibrium].ranks[1], probability: 0.025 },
                        PerkRankValuesProbabilityContainer { values: DATA.perks[PerkName::Equilibrium].ranks[2], probability: 0.0 },
                    ])
                }
            ];
            let actual = calc_perk_rank_probabilities(&*DATA, &partial_perk_values, true);
            assert_perk_values_eq(&actual, &expected);
        }
    }

    mod can_generate_wanted_ranks_test {
        use super::*;
        use crate::{stack_map::StackMap, stack_vec::StackVec};

        static DATA: Lazy<Data> = Lazy::new(|| {
            Data {
                comps: StackMap::new(),
                perks: {
                    let mut map = StackMap::new();
                    map.insert(PerkName::Empty, PerkRanksData {
                        doubleslot: false,
                        ranks: StackVec::new(&[
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                        ])
                    });
                    map.insert(PerkName::Precise, PerkRanksData {
                        doubleslot: false,
                        ranks: StackVec::new(&[
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                            PerkRankValues { rank: 1, threshold: 10, ..Default::default() },
                            PerkRankValues { rank: 2, threshold: 100, ..Default::default() },
                            PerkRankValues { rank: 3, threshold: 150, ..Default::default() },
                        ])
                    });
                    map.insert(PerkName::Biting, PerkRanksData {
                        doubleslot: false,
                        ranks: StackVec::new(&[
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                            PerkRankValues { rank: 1, threshold: 50, ..Default::default() },
                            PerkRankValues { rank: 2, threshold: 80, ..Default::default() },
                            PerkRankValues { rank: 3, threshold: 200, ..Default::default() },
                            PerkRankValues { rank: 4, threshold: 250, ..Default::default() },
                        ])
                    });
                    map.insert(PerkName::Equilibrium, PerkRanksData {
                        doubleslot: false,
                        ranks: StackVec::new(&[
                            PerkRankValues { rank: 0, threshold: 0, ..Default::default() },
                            PerkRankValues { rank: 1, threshold: 50, ..Default::default() },
                            PerkRankValues { rank: 2, threshold: 80, ..Default::default() },
                            PerkRankValues { rank: 3, threshold: 200, ..Default::default() },
                            PerkRankValues { rank: 4, threshold: 250, ..Default::default() },
                        ])
                    });
                    map
                }
            }
        });

        #[test]
        fn single_wanted_not_in_perk_values() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 50, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Equilibrium, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Empty, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn first_wanted_not_in_perk_values() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 50, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Equilibrium, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn second_wanted_not_in_perk_values() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 50, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Equilibrium, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn single_wanted_pv_below_threshold() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[20, 71]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Empty, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn first_wanted_pv_below_threshold() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[20, 71]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Biting, rank: 1, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn second_wanted_pv_below_threshold() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 10, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Biting, rank: 1, ..Default::default() },
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn single_wanted_pv_above_threshold() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 50, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 12, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Biting, rank: 1, ..Default::default() },
                    Perk { name: PerkName::Empty, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(true, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn both_wanted_pv_above_threshold() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 50, rolls: StackVec::new(&[20, 40]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Biting, rank: 1, ..Default::default() },
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(true, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn first_wanted_pv_base_too_high() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 80, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 100, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Biting, rank: 1, ..Default::default() },
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }

        #[test]
        fn second_wanted_pv_base_too_high() {
            let perk_values_arr = smallvec![
                PartialPerkValues { name: PerkName::Precise, base: 160, rolls: StackVec::new(&[20, 20]) },
                PartialPerkValues { name: PerkName::Biting, base: 50, rolls: StackVec::new(&[20, 20]) },
            ];
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Biting, rank: 1, ..Default::default() },
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                ),
                ..Default::default()
            };
            assert_eq!(false, can_generate_wanted_ranks(&*DATA, &perk_values_arr, wanted_gizmo))
        }
    }

    mod permutate_perk_ranks_test {
        use super::*;
        use smallvec::smallvec;

        fn assert_rank_combination_eq(actual: &Vec<RankCombination>, expected: &Vec<RankCombination>) {
            check_len(actual, expected);

            for x in expected {
                assert!(actual.contains(&x), "Actual doesn't contain {:#?}", x);
            }
        }

        #[test]
        fn permutate_ranks() {
            let perk_list: PerkValuesVec = smallvec![
                PerkValues {
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, name: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, name: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, name: PerkName::Precise, ..Default::default() }},
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 2,
                    i_last: 3,
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, name: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, name: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, name: PerkName::Biting, ..Default::default() }},
                    ]),
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

        static PERK_LIST: Lazy<PerkValuesVec> = Lazy::new(|| {
            smallvec![
                PerkValues {
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 3,
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, name: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, name: PerkName::Precise, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, name: PerkName::Precise, ..Default::default() }},
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 2,
                    i_last: 3,
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.125, values: PerkRankValues { rank: 1, name: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 2, name: PerkName::Biting, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 3, name: PerkName::Biting, ..Default::default() }},
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Equilibrium,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PerkRankValuesProbabilityContainer { probability: 0.0, values: PerkRankValues { rank: 0, name: PerkName::Equilibrium, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.25, values: PerkRankValues { rank: 1, name: PerkName::Equilibrium, ..Default::default() }},
                        PerkRankValuesProbabilityContainer { probability: 0.5, values: PerkRankValues { rank: 2, name: PerkName::Equilibrium, ..Default::default() }},
                    ]),
                    ..Default::default()
                },
            ]
        });

        #[test]
        fn permutate_ranks_one_wanted() {
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Empty, ..Default::default() },
                ),
                ..Default::default()
            };
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
            let actual = permutate_perk_ranks(&*PERK_LIST, Some(wanted_gizmo));
            assert_rank_combination_eq(&actual, &expected);
        }

        #[test]
        fn permutate_ranks_two_wanted() {
            let wanted_gizmo = Gizmo {
                perks: (
                    Perk { name: PerkName::Precise, rank: 2, ..Default::default() },
                    Perk { name: PerkName::Equilibrium, rank: 1, ..Default::default() },
                ),
                ..Default::default()
            };
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
            let actual = permutate_perk_ranks(&*PERK_LIST, Some(wanted_gizmo));
            assert_rank_combination_eq(&actual, &expected);
        }
    }

    mod get_empty_gizmo_chance_tests {
        use super::*;
        use approx::assert_relative_eq;

        static BUDGET: Lazy<Budget> = Lazy::new(||
            Budget {
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
            }
        );

        #[test]
        fn all_have_zero_rank_one_has_cost_below_min_range() {
            let pv = vec![
                PerkValues {
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 40, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 50, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 20, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 20, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 150, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 20, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 0,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 10, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 15, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
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
                    name: PerkName::Precise,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 1, cost: 100, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Precise, rank: 2, cost: 150, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
                PerkValues {
                    name: PerkName::Biting,
                    i_first: 1,
                    i_last: 2,
                    ranks: StackVec::new(&[
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 0, cost: 0, ..Default::default() }, probability: 0.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 1, cost: 15, ..Default::default() }, probability: 1.0/8.0 },
                        PRVPC { values: PerkRankValues { name: PerkName::Biting, rank: 2, cost: 200, ..Default::default() }, probability: 6.0/8.0 },
                    ]),
                    ..Default::default()
                },
            ];
            let expected = 0.65625;
            let actual = get_empty_gizmo_chance(&*BUDGET, &pv);
            assert_relative_eq!(actual, expected);
        }
    }
}