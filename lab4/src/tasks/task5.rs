use std::{
    cmp::{max, min},
    u32,
};

pub fn run() {
    let config = input();
    println!("{}", solve_enumeration(config));
}

impl Config {
    pub fn new(number_copies: u32, copier_scanner1: u32, copier_scanner2: u32) -> Self {
        Self {
            number_copies,
            copier_scanner1,
            copier_scanner2,
        }
    }
}

pub struct Config {
    number_copies: u32,
    copier_scanner1: u32,
    copier_scanner2: u32,
}

pub fn solve_enumeration(config: Config) -> u32 {
    let fast_scanner = min(config.copier_scanner1, config.copier_scanner2);
    let slow_scanner = max(config.copier_scanner1, config.copier_scanner2);

    let number_copies = config.number_copies - 1; // Первую копию делаем на быстром ксероксе
    let mut best_time = number_copies * fast_scanner;

    for for_slow in 1..=(number_copies / 2 + 1) {
        let for_fast = number_copies - for_slow;
        let time = max(fast_scanner * for_fast, slow_scanner * for_slow);

        if time < best_time {
            best_time = time;
        }
    }

    best_time + fast_scanner // Вот она - первая копия
}

fn input() -> Config {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    Config {
        number_copies: iter.next().unwrap().parse().unwrap(),
        copier_scanner1: iter.next().unwrap().parse().unwrap(),
        copier_scanner2: iter.next().unwrap().parse().unwrap(),
    }
}

pub fn solve_math(config: Config) -> u32 {
    let fast_scanner = config.copier_scanner1.min(config.copier_scanner2);
    let slow_scanner = config.copier_scanner1.max(config.copier_scanner2);

    let number_copies = config.number_copies - 1; // Первую копию делаем на быстром ксероксе

    // Оптимальное количество копий для медленного сканера
    let optimal_for_slow = (number_copies * fast_scanner) / (fast_scanner + slow_scanner);

    // Рассматриваем диапазон значенией
    let start = optimal_for_slow.saturating_sub(1);
    let end = min(optimal_for_slow + 1, number_copies);

    let mut best_time = number_copies * fast_scanner;

    for for_slow in start..=end {
        let for_fast = number_copies - for_slow;
        let time = (fast_scanner * for_fast).max(slow_scanner * for_slow);

        if time < best_time {
            best_time = time;
        }
    }

    best_time + fast_scanner // Добавляем время первой копии
}

pub fn solve_binary(config: Config) -> u32 {
    let fast_scanner = config.copier_scanner1.min(config.copier_scanner2);
    let slow_scanner = config.copier_scanner1.max(config.copier_scanner2);

    let number_copies = config.number_copies - 1; // Первую копию делаем на быстром ксероксе

    let mut left = 0;
    let mut right = number_copies / 2 + 1;
    let mut best_time = fast_scanner * number_copies;

    while left <= right {
        let for_slow = (left + right) / 2;
        let for_fast = number_copies - for_slow;
        let current_time = (fast_scanner * for_fast).max(slow_scanner * for_slow);

        if current_time < best_time {
            best_time = current_time;
        }

        // Определяем направление поиска
        if fast_scanner * for_fast > slow_scanner * for_slow {
            left = for_slow + 1; // Нужно больше копий на медленном
        } else {
            right = for_slow - 1; // Нужно меньше копий на медленном
        }
    }

    best_time + fast_scanner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_task5() {
        test_helper(4, 1, 1, 3);
        test_helper(5, 1, 2, 4);
        test_helper(9, 1, 10, 9);

        // Equal scanners
        test_helper(3, 2, 2, 4);
        test_helper(10, 5, 5, 30);

        // Different scanners
        test_helper(6, 2, 3, 8);
        test_helper(8, 3, 1, 7);

        test_helper(100, 1, 1, 51);
        test_helper(100, 2, 1, 67);
        test_helper(100, 3, 1, 76);
        test_helper(100, 10, 1, 91);
        test_helper(100, 10, 5, 335);
        test_helper(100, 10, 10, 510);
        test_helper(1000, 7, 8, 3738);
    }

    fn test_helper(number_copies: u32, copier_scanner1: u32, copier_scanner2: u32, expected: u32) {
        let actual = solve_binary(Config {
            number_copies,
            copier_scanner1,
            copier_scanner2,
        });
        assert_eq!(
            actual, expected,
            "Failed with input: copies={}, scanner1={}, scanner2={}",
            number_copies, copier_scanner1, copier_scanner2
        );
    }
}
