/// Binomial coefficient $\frac{n!}{k!(n-k)!}$
pub fn choose(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0
    }

    let mut res = 1.0;
    let mut n = n as f64;
    for d in (1..=k).rev() {
        res *= n / d as f64;
        n -= 1.0;
    }

    res
}

/// Calculates the chance to roll `val` with `die_count` amount of dice where each die has `die_sides` faces.
///
/// https://mathworld.wolfram.com/Dice.html
pub fn dice_roll(val: usize, die_count: usize, die_sides: usize) -> f64 {
    let mut res = 0.0;

    if die_sides == 0 {
        return 0.0
    }

    for k in 0..=((val - die_count)/die_sides) {
        let sign = if k % 2 == 0 { 1f64 } else { -1f64 };
        res += sign * choose(die_count, k) * choose(val - die_sides*k - 1, die_count - 1);
    }

    res / (die_sides as f64).powf(die_count as f64)
}

/// Returns a multinomial distribution indication the probability to see a certain count when
/// summing the results of rolling a discrete uniform distribution ranging from 0 to `range` (exclusive), `rolls` times
pub fn get_distribution(range: usize, rolls: usize) -> Vec<f64> {
    if rolls == 0 || range == 0 {
        return vec![];
    }

    if rolls == 1 {
        let x = 1.0 / range as f64;
        return vec![x; range];
    }

    let mut dist = Vec::with_capacity(range * rolls);
    for i in 0..=((range - 1) * rolls) {
        dist.push(dice_roll(i + rolls, rolls, range))
    }

    dist
}

/// Cumulative multinomial distribution
pub fn get_cumulative_distribution(range: usize, rolls: usize) -> Vec<f64> {
    let dist = get_distribution(range, rolls);
    let mut res = Vec::with_capacity(dist.len());
    let mut sum = 0.0;

    for x in dist.iter() {
        sum += x;
        res.push(sum);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use approx::*;

    #[test_case(0, 0 => is almost 1.0 precision f64::EPSILON)]
    #[test_case(0, 5 => is almost 0.0 precision f64::EPSILON)]
    #[test_case(5, 0 => is almost 1.0 precision f64::EPSILON)]
    #[test_case(5, 5 => is almost 1.0 precision f64::EPSILON)]
    #[test_case(5, 2 => is almost 10.0 precision f64::EPSILON)]
    fn choose_tests(n: usize, k: usize) -> f64 {
        choose(n, k)
    }

    #[test_case(0, 0, 0 => is almost 0.0 precision f64::EPSILON)]
    #[test_case(1, 1, 6 => is almost 1.0/6.0 precision f64::EPSILON)]
    #[test_case(2, 2, 6 => is almost 1.0/36.0 precision f64::EPSILON)]
    #[test_case(6, 2, 6 => is almost 5.0/36.0 precision f64::EPSILON)]
    #[test_case(20, 5, 10 => is almost 3246.0/100000.0 precision f64::EPSILON)]
    fn dice_roll_test(val: usize, count: usize, sides: usize) -> f64 {
        dice_roll(val, count, sides)
    }

    fn assert_relative_eq_vec_(actual: &Vec<f64>, expected: &Vec<f64>, max_relative: f64)
    {
        assert!(actual.len() == expected.len(), "Vectors have different lengths (#actual: {}, #expected: {})\n", actual.len(), expected.len());

        for (i, x) in actual.iter().enumerate() {
            let res = relative_eq!(x, &expected[i], max_relative = max_relative * f64::EPSILON);
            assert!(res, "Elements at intex {} are not equal (actual: '{}', expected: '{}')\n", i, x, expected[i]);
        }
    }

    fn assert_relative_eq_vec(actual: &Vec<f64>, expected: &Vec<f64>) {
        assert_relative_eq_vec_(actual, expected, 1.0)
    }

    mod get_distribution_tests {
        use super::*;

        #[test]
        fn range_rolls_0_0() {
            let res = get_distribution(0, 0);
            assert!(res.len() == 0);
        }

        #[test]
        fn range_rolls_10_1() {
            let expected = vec![0.1; 10];
            assert_relative_eq_vec(&get_distribution(10, 1), &expected);
        }

        #[test]
        fn range_rolls_10_2() {
            let expected = vec![
                0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.09, 0.08, 0.07, 0.06,
                0.05, 0.04, 0.03, 0.02, 0.01
            ];
            assert_relative_eq_vec(&get_distribution(10, 2), &expected);
        }

        #[test]
        fn range_rolls_10_4() {
            let expected = vec![
                0.0001, 0.0004, 0.001, 0.002, 0.0035, 0.0056, 0.0084, 0.012, 0.0165, 0.022,
                0.0282, 0.0348, 0.0415, 0.048, 0.054, 0.0592, 0.0633, 0.066, 0.067, 0.066,
                0.0633, 0.0592, 0.054, 0.048, 0.0415, 0.0348, 0.0282, 0.022, 0.0165, 0.012,
                0.0084, 0.0056, 0.0035, 0.002, 0.001, 0.0004, 0.0001
            ];
            assert_relative_eq_vec(&get_distribution(10, 4), &expected);
        }
    }

    mod get_cumulative_distribution_tests {
        use super::*;

        #[test]
        fn range_rolls_0_0() {
            let res = get_cumulative_distribution(0, 0);
            assert!(res.len() == 0);
        }

        #[test]
        fn range_rolls_10_1() {
            let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
            assert_relative_eq_vec(&get_cumulative_distribution(10, 1), &expected);
        }

        #[test]
        fn range_rolls_10_2() {
            let expected = vec![
                0.01, 0.03, 0.06, 0.1, 0.15, 0.21, 0.28, 0.36, 0.45, 0.55, 0.64, 0.72, 0.79,
                0.85, 0.9, 0.94, 0.97, 0.99, 1.0
            ];
            assert_relative_eq_vec(&get_cumulative_distribution(10, 2), &expected);
        }

        #[test]
        fn range_rolls_10_4() {
            let expected = vec![
                0.0001, 0.0005, 0.0015, 0.0035, 0.007, 0.0126, 0.021, 0.033, 0.0495,
                0.0715, 0.0997, 0.1345, 0.176, 0.224, 0.278, 0.3372, 0.4005, 0.4665,
                0.5335, 0.5995, 0.6628, 0.722, 0.776, 0.824, 0.8655, 0.9003, 0.9285,
                0.9505, 0.967, 0.979, 0.9874, 0.993, 0.9965, 0.9985, 0.9995, 0.9999, 1.0
            ];
            assert_relative_eq_vec_(&get_cumulative_distribution(10, 4), &expected, 5.0);
        }
    }
}