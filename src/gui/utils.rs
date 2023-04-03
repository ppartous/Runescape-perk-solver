#![allow(unused)]

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
    if s.ends_with(',') {
        s.pop();
    }
    if is_negative {
        s.push('-');
    }
    s.chars().rev().collect()
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
}