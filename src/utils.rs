use std::process;
use colored::*;
use approx;
use len_trait;

use crate::RankCombination;

pub fn print_error(err: &str) -> ! {
    eprintln!("{}{} {err}", "error".red().bold(), ":".bold());
    process::exit(0)
}

pub fn format_int(mut num: i64) -> String {
    let mut s = String::new();
    let is_negative = num.is_negative();
    num = num.abs();
    for (i, char) in num.to_string().chars().rev().enumerate() {
        s.push(char);
        if (i + 1) % 3 == 0 {
            s.push(',');
        }
    }
    if s.chars().last().unwrap() == ',' {
        s.pop();
    }
    if is_negative {
        s.push('-');
    }
    s.chars().rev().collect()
}

/// Convolutes two arrays <https://en.wikipedia.org/wiki/Convolution>
pub fn convolve(x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
    let xlen = x.len() as i64;
    let ylen = y.len() as i64;
    let mut z = Vec::with_capacity((xlen + ylen - 1) as usize);
    for i in 0..=(xlen + ylen - 2) {
        let start = i64::max(0, i - (ylen - 1)) as usize;
        let stop = i64::min(xlen - 1, i) as usize;
        let sum = x[start..=stop].iter().zip(y[(i as usize - stop)..=(i as usize - start)].iter().rev())
            .fold(0.0, |acc, (xval, yval)| acc + xval * yval);
        z.push(sum);
    }
    z
}

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

pub fn check<T>(acc: T, exp: T, name: &str)
where
    T: PartialEq + std::fmt::Display
{
    if acc != exp {
        eprintln!("Actual and expected have different '{}' values (actual: {}, expected: {})", name, acc, exp);
        panic!("value differs");
    }
}

pub fn check_len<T>(acc: &T, exp: &T)
where
    T: std::fmt::Debug + len_trait::Len
{
    if acc.len() != exp.len() {
        eprintln!("Actual and expected have different lengths (actual: {}, expected: {})\nActual: {:#?}\nExpected: {:#?}",
            acc.len(), exp.len(), acc, exp);
        panic!("value differs");
    }
}

pub fn check_index<T, K>(acc: T, exp: T, i: usize, name: &str, acc_full: K, exp_full: K)
where
    T: PartialEq + std::fmt::Display,
    K: std::fmt::Debug
{
    if acc != exp {
        eprintln!("Actual and expected have different '{}' values at index {} (actual: {}, expected: {})\nActual: {:#?}\nExpected: {:#?}",
            name, i, acc, exp, acc_full, exp_full);
        panic!("value differs");
    }
}

pub fn check_index_relative<K>(acc: f64, exp: f64, max_relative: f64, i: usize, name: &str, acc_full: K, exp_full: K)
where
    K: std::fmt::Debug
{
    if approx::relative_ne!(acc, exp, max_relative = max_relative * f64::EPSILON) {
        eprintln!("Actual and expected have different '{}' values at index {} (actual: {}, expected: {})\n\
            Delta: {} > {} \nActual: {:#?}\nExpected: {:#?}",
            name, i, acc, exp, acc - exp, max_relative * f64::EPSILON, acc_full, exp_full);
        panic!("value differs");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0 => "0")]
    #[test_case(10 => "10")]
    #[test_case(100 => "100")]
    #[test_case(1000 => "1,000")]
    #[test_case(10000 => "10,000")]
    #[test_case(100000 => "100,000")]
    #[test_case(1000000 => "1,000,000")]
    #[test_case(10000000 => "10,000,000")]
    #[test_case(100000000 => "100,000,000")]
    #[test_case(1000000000 => "1,000,000,000")]
    #[test_case(10000000000 => "10,000,000,000")]
    #[test_case(-10 => "-10"; "negative 10")]
    #[test_case(-100 => "-100"; "negative 100")]
    #[test_case(-1000 => "-1,000"; "negative 1000")]
    #[test_case(-10000 => "-10,000"; "negative 10000")]
    fn fmt_int(x: i64) -> String {
        format_int(x)
    }

    #[test_case(vec![1.0; 6], vec![1.0; 3] => it contains_in_order [1.0, 2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 1.0])]
    #[test_case(vec![1.0; 6], vec![1.0; 3] => with |x: Vec<f64>| assert!(x.len() == 8))]
    #[test_case(vec![1.0; 3], vec![1.0; 6] => it contains_in_order [1.0, 2.0, 3.0, 3.0, 3.0, 3.0, 2.0, 1.0])]
    #[test_case(vec![1.0; 3], vec![1.0; 6] => with |x: Vec<f64>| assert!(x.len() == 8))]
    fn convolve_test(x: Vec<f64>, y: Vec<f64>) -> Vec<f64> {
        convolve(&x, &y)
    }

    mod quicksort_tests {
        use super::*;
        use smallvec::smallvec;
        use crate::{PerkName, PerkRankValues};

        fn assert_rankcombination_eq(actual: &RankCombination, expected: &RankCombination) {
            PerkName::using_simplified_names();
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
}