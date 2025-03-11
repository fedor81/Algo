use std::{
    char::{self, from_digit},
    collections::HashMap,
    io,
};

pub fn run() {
    let (key_count, board) = input();
    let result = solve(key_count, &board);
    println!("{result}");
}

fn input() -> (i32, [[char; 4]; 4]) {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin
        .read_line(&mut input)
        .expect("Невозможно получить ввод");

    let key_count = input.trim().parse::<i32>().expect("Неверный формат числа") * 2; // Два игрока, умножаем на два

    let board = convert_to_board((0..4).map(|_| {
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();
        if input.len() != 4 {
            panic!("Неверный формат ввода");
        }
        input.to_string()
    }));

    (key_count, board)
}

fn convert_to_board<T: IntoIterator<Item = String>>(input_board: T) -> [[char; 4]; 4] {
    let mut board = [['.'; 4]; 4];

    for (i, input) in input_board.into_iter().enumerate() {
        for (j, c) in input.chars().enumerate() {
            if c.is_digit(10) && c != '0' || c == '.' {
                board[i][j] = c;
            } else {
                panic!("Неверный ввод: {c}")
            }
        }
    }
    board
}

fn solve(key_count: i32, board: &[[char; 4]; 4]) -> i32 {
    let mut hash_map = HashMap::new();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let c = board[i][j];
            if !hash_map.contains_key(&c) {
                hash_map.insert(c, 1);
            } else {
                *hash_map.get_mut(&c).unwrap() += 1;
            }
        }
    }

    let mut result = 0;

    for t in 1..10 {
        let current_char =
            &from_digit(t, 10).expect(&format!("Не получилось преобразовать в char {}", t));
        if let Some(count) = hash_map.get(current_char) {
            if *count <= key_count {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_cases() {
        test_helper(2, vec!["1111", "1111", "1111", "1111"], 0);
        test_helper(4, vec!["1111", "....", "....", "...."], 1);
        test_helper(4, vec!["1111", "2222", "3333", "44.."], 4);
        test_helper(9, vec!["1111", "1111", "1444", "4444"], 2);
        test_helper(9, vec!["....", "....", "....", "...."], 0);
        test_helper(2, vec!["1234", "5678", "9123", "4567"], 9);
        test_helper(1, vec!["1234", "5678", "9123", "4567"], 2);
    }

    fn test_helper(key_count: i32, board: Vec<&str>, expected: i32) {
        let board = convert_to_board(board.iter().map(|s| s.to_string()));
        let result = solve(key_count, &board);
        assert_eq!(result, expected);
    }
}
