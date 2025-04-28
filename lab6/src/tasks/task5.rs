use std::collections::{HashMap, HashSet};

pub fn run() {
    let (a, b) = input();
    let result = decimal_representation(a, b);
    println!("{}", result);
}

fn input() -> (u32, u32) {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let numbers: Vec<u32> = buf
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    (numbers[0], numbers[1])
}

fn decimal_representation(numerator: u32, denominator: u32) -> String {
    let integer_part = numerator / denominator;
    let mut remainder = numerator % denominator;

    let mut remainder_digits = vec![];
    let mut repeating_remainders = HashMap::new();
    let mut repeat = None;

    while remainder != 0 {
        if let Some(&position) = repeating_remainders.get(&remainder) {
            repeat = Some(position);
            break;
        }

        repeating_remainders.insert(remainder, remainder_digits.len());
        remainder *= 10;
        remainder_digits.push(remainder / denominator);
        remainder %= denominator;
    }

    let mut result = String::from(integer_part.to_string());
    result.push('.');

    if let Some(position) = repeat {
        for (i, digit) in remainder_digits.into_iter().enumerate() {
            if i == position {
                result.push('(');
            }
            result.push_str(digit.to_string().as_str());
        }
        result.push(')');
    } else {
        for digit in remainder_digits {
            result.push_str(digit.to_string().as_str());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_representation() {
        test_helper(1, 2, "0.5");
        test_helper(5, 4, "1.25");
        test_helper(17, 250, "0.068");
        test_helper(999999, 1000000, "0.999999");
        test_helper(1, 3, "0.(3)");
        test_helper(2, 3, "0.(6)");
        test_helper(4, 333, "0.(012)");
        test_helper(1, 6, "0.1(6)");
        test_helper(1, 7, "0.(142857)");
        test_helper(6, 70, "0.0(857142)");
    }

    fn test_helper(numerator: u32, denominator: u32, expected: &str) {
        assert_eq!(
            decimal_representation(numerator, denominator),
            expected,
            "({}, {})",
            numerator,
            denominator
        );
    }
}
