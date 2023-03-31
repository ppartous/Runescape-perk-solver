use crate::RankCombination;

pub fn jagex_quicksort(rank_combination: &mut RankCombination) {
    fn f(comb: &mut RankCombination, low: usize, high: usize) {
        let ranks = &mut comb.ranks;
        let pivot_index = (low + high) / 2;
        let pivot_value = ranks[pivot_index];
        ranks.swap(pivot_index, high);
        let mut counter = low;

        for i in low..high {
            if (ranks[i].cost as i64 - pivot_value.cost as i64) < (i as i64 & 1) {
                ranks.swap(i, counter);
                counter += 1;
            }
        }

        ranks.swap(high, counter);

        if (low as i64) < (counter as i64 - 1) {
            f(comb, low, counter - 1);
        }
        if (counter + 1) < high {
            f(comb, counter + 1, high);
        }
    }

    f(rank_combination, 0, rank_combination.ranks.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;
    use crate::{PerkName, PerkRankValues, utils::check_index};

    fn assert_rankcombination_eq(actual: &RankCombination, expected: &RankCombination) {
        for (i, (acc, exp)) in actual.ranks.iter().zip(expected.ranks.iter()).enumerate() {
            check_index(acc.name, exp.name, i, "perk", actual, expected);
            check_index(acc.cost, exp.cost, i, "cost", actual, expected);
        }
    }

    #[test]
    fn even_no_equal_costs() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 20, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 20, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs1() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs2() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs3() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs4() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs5() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs6() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs7() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs8() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs9() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs10() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs11() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs12() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn even_equal_costs13() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_no_equal_costs() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 20, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 20, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs1() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs2() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs3() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs4() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs5() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs6() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }

    #[test]
    fn uneven_equal_costs7() {
        let mut t = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
            ],
            probability: 0.0
        };
        let expected = RankCombination {
            ranks: smallvec![
                PerkRankValues { name: PerkName::A, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::B, cost: 10, ..Default::default() },
                PerkRankValues { name: PerkName::C, cost: 30, ..Default::default() },
                PerkRankValues { name: PerkName::D, cost: 40, ..Default::default() },
                PerkRankValues { name: PerkName::E, cost: 50, ..Default::default() },
            ],
            probability: 0.0
        };
        jagex_quicksort(&mut t);
        assert_rankcombination_eq(&t, &expected);
    }
}