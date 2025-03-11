use core::panic;
use std::cmp::min;

pub fn run() {
    let (count, weights) = input();
    let result = solve(count, weights);
    println!("{}", result);
}

fn input() -> (u8, Vec<i32>) {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let count: u8 = buf.trim().parse().expect(&format!("Неверный ввод {}", buf));

    if !(2 <= count && count <= 23) {
        panic!("N должно принадлежать [2, 23]")
    }

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let weights: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    (count, weights)
}

fn solve(count: u8, weights: Vec<i32>) -> u32 {
    find_min_diff(&weights, 0, 0) as u32
}

fn find_min_diff(weights: &[i32], sum1: i32, sum2: i32) -> i32 {
    if weights.len() == 0 {
        return (sum1 - sum2).abs();
    }

    let current = weights[0];

    min(
        find_min_diff(&weights[1..], sum1 + current, sum2),
        find_min_diff(&weights[1..], sum1, sum2 + current),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test12() {
        let (count, weights) = (2, vec![1, 2]);
        let result = solve(count, weights);
        assert_eq!(result, 1);
    }

    #[test]
    fn test123() {
        let (count, weights) = (3, vec![1, 2, 3]);
        let result = solve(count, weights);
        assert_eq!(result, 0);
    }

    #[test]
    fn test1236() {
        let (count, weights) = (4, vec![1, 2, 3, 6]);
        let result = solve(count, weights);
        assert_eq!(result, 0);
    }

    #[test]
    fn test15717() {
        let (count, weights) = (5, vec![1, 5, 7, 1, 7]);
        let result = solve(count, weights);
        assert_eq!(result, 3);
    }

    #[test]
    fn test89698() {
        let (count, weights) = (5, vec![8, 9, 6, 9, 8]);
        let result = solve(count, weights);
        assert_eq!(result, 4);
    }

    #[test]
    fn test14_2_12_9_9_8() {
        let (count, weights) = (6, vec![14, 2, 12, 9, 9, 8]);
        let result = solve(count, weights);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_big_numbers() {
        let (count, weights) = (3, vec![1111111, 5555555, 888888888]);
        let result = solve(count, weights);
        assert_eq!(result, 882222222);
    }
}
