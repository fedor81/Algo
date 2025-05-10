pub fn run() {
    let railways = input();
    let is_optimal = solve(&railways);

    println!("");
    if is_optimal {
        println!("YES");
    } else {
        println!("NO");
    }
}

pub fn solve(railways: &Vec<Vec<Railway>>) -> bool {
    let cities = railways.len();
    let mut can_reach_r = vec![vec![false; cities]; cities];
    let mut can_reach_b = vec![vec![false; cities]; cities];

    // Заводим все прямые пути
    for from in 0..cities {
        for to in (from + 1)..cities {
            match railways[from][to] {
                Railway::R => can_reach_r[from][to] = true,
                Railway::B => can_reach_b[from][to] = true,
                Railway::None => {}
            }
        }
    }

    // Теперь проверяем все пути с промежуточными городами
    for from in 0..cities {
        for to in (from + 2)..cities {
            for intermediate in (from + 1)..to {
                if can_reach_r[from][intermediate] && can_reach_r[intermediate][to] {
                    can_reach_r[from][to] = true;
                }
                if can_reach_b[from][intermediate] && can_reach_b[intermediate][to] {
                    can_reach_b[from][to] = true;
                }
            }

            match (can_reach_b[from][to], can_reach_r[from][to]) {
                (true, true) => return false,
                _ => (),
            }
        }
    }
    true
}

fn input() -> Vec<Vec<Railway>> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let count = buf.trim().parse::<usize>().unwrap();
    let mut railways = vec![vec![Railway::None; count]; count];

    for i in 0..(count - 1) {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        for (j, c) in buf.trim().chars().enumerate() {
            railways[i][i + 1 + j] = Railway::from_char(c);
        }
    }

    railways
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Railway {
    R,
    B,
    None,
}

impl Railway {
    pub fn from_char(c: char) -> Self {
        match c {
            'R' => Railway::R,
            'B' => Railway::B,
            _ => Railway::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    // Вспомогательная функция для эмуляции ввода из строки
    fn mock_input(input: &str) -> Vec<Vec<Railway>> {
        let mut cursor = Cursor::new(input);
        let mut buf = String::new();

        cursor.read_line(&mut buf).unwrap();
        let count = buf.trim().parse::<usize>().unwrap();
        let mut railways = vec![vec![Railway::None; count]; count];

        for i in 0..(count - 1) {
            buf.clear();
            cursor.read_line(&mut buf).unwrap();

            for (j, c) in buf.trim().chars().enumerate() {
                railways[i][i + 1 + j] = Railway::from_char(c);
            }
        }

        railways
    }

    #[test]
    fn test_input_n3() {
        let input = "3\nRB\nR\n";
        let railways = mock_input(input);

        assert_eq!(railways[0][1], Railway::R); // 1→2
        assert_eq!(railways[0][2], Railway::B); // 1→3
        assert_eq!(railways[1][2], Railway::R); // 2→3

        let actual = solve(&railways);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_input_n2() {
        let input = "2\nB\n";
        let railways = mock_input(input);

        assert_eq!(railways[0][1], Railway::B); // 1→2
        let actual = solve(&railways);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_input_n4() {
        let input = "4\nBBB\nRB\nB\n";
        let railways = mock_input(input);

        assert_eq!(railways[0][1], Railway::B); // 1→2
        assert_eq!(railways[0][2], Railway::B); // 1→3
        assert_eq!(railways[0][3], Railway::B); // 1→4
        assert_eq!(railways[1][2], Railway::R); // 2→3
        assert_eq!(railways[1][3], Railway::B); // 2→4
        assert_eq!(railways[2][3], Railway::B); // 3→4

        let actual = solve(&railways);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_with_input_n5() {
        let input = "5\nRRRB\nBRR\nBR\nR\n";
        let railways = mock_input(input);
        let actual = solve(&railways);
        assert_eq!(actual, false);
    }

    #[test]
    fn test_all_r_optimal() {
        // Входные данные:
        // 3
        // RR
        // R
        let railways = vec![
            vec![Railway::R, Railway::R],
            vec![Railway::None, Railway::R],
        ];
        assert_eq!(solve(&railways), true); // Ожидается YES, так как все пути типа R
    }

    #[test]
    fn test_conflict_in_middle() {
        // Входные данные:
        // 4
        // RBR
        // RB
        // R
        let railways = vec![
            vec![Railway::R, Railway::B, Railway::R],
            vec![Railway::None, Railway::R, Railway::B],
            vec![Railway::None, Railway::None, Railway::R],
        ];
        assert_eq!(solve(&railways), false); // Ожидается NO, так как есть конфликт для (1,4)
    }
}
