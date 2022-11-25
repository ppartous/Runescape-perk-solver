use std::process;
use colored::*;

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

/// Convolutes two arrays https://en.wikipedia.org/wiki/Convolution
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