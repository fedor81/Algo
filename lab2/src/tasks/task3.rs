use crate::modules::merge_sort::merge_sort;
use std::cmp::max;

pub fn run() {
    let numbers = input();
    let result = solve(numbers);
    println!("{}", result);
}

fn input() -> Vec<i32> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let count = buf.trim().parse::<u32>().unwrap();
    let mut numbers = Vec::new();

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let number = buf.trim().parse::<i32>().unwrap();
        numbers.push(number);
    }

    numbers
}

pub fn solve(mut numbers: Vec<i32>) -> i128 {
    if numbers.len() == 0 {
        return 0;
    } else if numbers.len() <= 3 {
        let mut result: i128 = 1;

        for number in numbers {
            result *= number as i128;
        }
        return result;
    }

    numbers = merge_sort(&mut numbers, false);

    let mul_biggest = numbers[numbers.len() - 1] as i128
        * numbers[numbers.len() - 2] as i128
        * numbers[numbers.len() - 3] as i128;

    let mul_smallest = numbers[0] as i128 * numbers[1] as i128 * numbers[numbers.len() - 1] as i128;
    max(mul_smallest, mul_biggest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        test_helper(vec![-1, 2, 3, -4, -2, 5, -1, 5, -3, -2], 75);
        test_helper(vec![-10, -8, 0, 6, 3], 480);
        test_helper(vec![-10, -11, 0, -99, -999], 0);
        test_helper(vec![0, 0, 0, 1, 1, 0, 0, 0], 0);
        test_helper(vec![0, 1, 0, -1, 1, 0, 0, 1], 1);
        test_helper(vec![1, -1, 1, -1, 1, -1, 2], 2);
        test_helper(vec![-100, -1000, -10, -5, -999, -3, -1, -998, -2], -6);
        test_helper(vec![10; 1000], 1000);
        test_helper(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 720);
        test_helper(vec![-10, -10, -10, 0, 1, 2, 3], 300);
    }

    fn test_helper(numbers: Vec<i32>, excepted: i128) {
        let result = solve(numbers);
        assert_eq!(result, excepted);
    }
}
