#![allow(unused)]
use std::process;
use colored::*;
use approx;
use len_trait;

pub fn print_error(err: &str) -> ! {
    eprintln!("{}{} {err}", "error".red().bold(), ":".bold());
    process::exit(0)
}

pub fn print_warning(err: &str) {
    eprintln!("{}{} {err}", "warning".yellow().bold(), ":".bold());
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

pub fn check_result<T>(acc: T, exp: T, name: &str) -> Result<(), String>
where
    T: PartialEq + std::fmt::Display
{
    if acc != exp {
        return Err(format!("Actual and expected have different '{}' values (actual: {}, expected: {})", name, acc, exp));
    }
    Ok(())
}

pub fn check<T>(acc: T, exp: T, name: &str)
where
    T: PartialEq + std::fmt::Display
{
    if let Err(err) = check_result(acc, exp, name) {
        panic!("{}", err);
    }
}

pub fn check_len_result<T>(acc: &T, exp: &T) -> Result<(), String>
where
    T: std::fmt::Debug + len_trait::Len
{
    if acc.len() != exp.len() {
        return Err(format!("Actual and expected have different lengths (actual: {}, expected: {})\nActual: {:#?}\nExpected: {:#?}",
            acc.len(), exp.len(), acc, exp));
    }
    Ok(())
}

pub fn check_len<T>(acc: &T, exp: &T)
where
    T: std::fmt::Debug + len_trait::Len
{
    if let Err(err) = check_len_result(acc, exp) {
        panic!("{}", err);
    }
}

pub fn check_index_result<T, K>(acc: T, exp: T, i: usize, name: &str, acc_full: K, exp_full: K) -> Result<(), String>
where
    T: PartialEq + std::fmt::Display,
    K: std::fmt::Debug
{
    if acc != exp {
        return Err(format!("Actual and expected have different '{}' values at index {} (actual: {}, expected: {})\nActual: {:#?}\nExpected: {:#?}",
            name, i, acc, exp, acc_full, exp_full));
    }
    Ok(())
}

pub fn check_index<T, K>(acc: T, exp: T, i: usize, name: &str, acc_full: K, exp_full: K)
where
    T: PartialEq + std::fmt::Display,
    K: std::fmt::Debug
{
    if let Err(err) = check_index_result(acc, exp, i, name, acc_full, exp_full) {
        panic!("{}", err);
    }
}

pub fn check_index_relative_result<K>(acc: f64, exp: f64, max_relative: f64, i: usize, name: &str, acc_full: K, exp_full: K) -> Result<(), String>
where
    K: std::fmt::Debug
{
    if approx::relative_ne!(acc, exp, max_relative = max_relative * f64::EPSILON) {
        return Err(format!("Actual and expected have different '{}' values at index {} (actual: {}, expected: {})\n\
            Delta: {} > {} \nActual: {:#?}\nExpected: {:#?}",
            name, i, acc, exp, acc - exp, max_relative * f64::EPSILON, acc_full, exp_full));
    }
    Ok(())
}

pub fn check_index_relative<K>(acc: f64, exp: f64, max_relative: f64, i: usize, name: &str, acc_full: K, exp_full: K)
where
    K: std::fmt::Debug
{
    if let Err(err) = check_index_relative_result(acc, exp, max_relative, i, name, acc_full, exp_full) {
        panic!("{}", err);
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
}