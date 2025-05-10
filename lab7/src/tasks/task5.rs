use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let grid = input(&mut reader);
    let result = solve(&grid);
    println!("{}", result);
}

fn solve(grid: &Vec<Vec<Cell>>) -> usize {
    let mut result = 0;
    let (width, height) = (grid[0].len(), grid.len());
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == Cell::Rug && !visited[i][j] {
                result += 1;
                visited[i][j] = true;
                queue.push_back((i, j));

                while let Some((i, j)) = queue.pop_front() {
                    for (i, j, neighbor) in get_neighbors(grid, i, j) {
                        if neighbor == Cell::Rug && !visited[i][j] {
                            queue.push_back((i, j));
                            visited[i][j] = true;
                        }
                    }
                }
            }
        }
    }

    result
}

/// Возвращает ближайших 4-х соседей
fn get_neighbors<T: Copy>(v: &Vec<Vec<T>>, i: usize, j: usize) -> Vec<(usize, usize, T)> {
    let mut neighbors = Vec::new();
    let rows = v.len();
    if rows == 0 {
        return neighbors;
    }
    let cols = v[0].len();
    let offset = 1_i32;

    for offset_i in -offset..=offset {
        for offset_j in -offset..=offset {
            let dis = offset_i.abs() + offset_j.abs();

            // Берем только четырех соседей
            if dis != 1 {
                continue;
            }

            let neighbor_i = i as i32 + offset_i;
            let neighbor_j = j as i32 + offset_j;

            if 0 <= neighbor_i && neighbor_i < rows as i32 && 0 <= neighbor_j && neighbor_j < cols as i32 {
                let neighbor_i = neighbor_i as usize;
                let neighbor_j = neighbor_j as usize;
                let neighbor = v[neighbor_i][neighbor_j];
                neighbors.push((neighbor_i, neighbor_j, neighbor));
            }
        }
    }

    neighbors
}

fn input<T: BufRead>(input: &mut T) -> Vec<Vec<Cell>> {
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();

    let sizes = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    let (width, height) = (sizes[0], sizes[1]);
    let mut grid = vec![vec![Cell::Empty; width]; height];

    for i in 0..height {
        buf.clear();
        input.read_line(&mut buf).unwrap();

        for (j, c) in buf.trim().chars().enumerate() {
            match c {
                '.' => grid[i][j] = Cell::Empty,
                '+' => grid[i][j] = Cell::Rug,
                _ => panic!("Invalid cell: {}", c),
            }
        }
    }
    grid
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Rug,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_neighbors() {
        let grid = vec![
            vec![Cell::Rug, Cell::Empty, Cell::Rug],
            vec![Cell::Empty, Cell::Rug, Cell::Empty],
            vec![Cell::Rug, Cell::Empty, Cell::Rug],
        ];

        assert_eq!(
            get_neighbors(&grid, 0, 2),
            vec![(0, 1, Cell::Empty), (1, 2, Cell::Empty)]
        );

        assert_eq!(
            get_neighbors(&grid, 1, 1),
            vec![
                (0, 1, Cell::Empty),
                (1, 0, Cell::Empty),
                (1, 2, Cell::Empty),
                (2, 1, Cell::Empty),
            ]
        );
    }

    #[test]
    fn test_solve2() {
        let input_str = "5 5
..+..
.+++.
.+..+
+.+.+
+++++";
        let grid = input(&mut Cursor::new(input_str));
        assert_eq!(solve(&grid), 2);
    }

    #[test]
    fn test_with_input() {
        let input_str = "6 6
.++++.
.....+
+..+..
..++..
.+....
.+...+";
        let grid = input(&mut Cursor::new(input_str));

        assert_eq!(grid.len(), 6);
        assert_eq!(grid[0].len(), 6);

        assert_eq!(grid[0][0], Cell::Empty);
        assert_eq!(grid[0][1], Cell::Rug);
        assert_eq!(grid[0][2], Cell::Rug);
        assert_eq!(grid[3][2], Cell::Rug);
        assert_eq!(grid[4][1], Cell::Rug);
        assert_eq!(grid[5][1], Cell::Rug);
        assert_eq!(grid[4][5], Cell::Empty);
        assert_eq!(grid[5][5], Cell::Rug);

        assert_eq!(solve(&grid), 6);
    }

    #[test]
    fn test_solve() {
        let input_str = "10 10
.++..++.++
+.+....+++
........++
+....++...
+++.+++.++
.....+.+..
+.+.++++++
+.++.++..+
++++...++.
+...+..+..";
        let grid = input(&mut Cursor::new(input_str));
        assert_eq!(solve(&grid), 10);
    }

    #[test]
    fn test_big_rug() {
        let count = 1000;
        let mut str = "+".repeat(count) + "\n";
        str = str.repeat(count);
        str = format!("{} {}\n", count, count) + &str;
        let grid = input(&mut Cursor::new(str));
        assert_eq!(solve(&grid), 1);
    }

    #[test]
    fn test_single_cell() {
        let input_str = "1 1\n+";
        let grid = input(&mut Cursor::new(input_str));
        assert_eq!(solve(&grid), 1);
    }

    #[test]
    fn test_no_rugs() {
        let input_str = "3 3\n...\n...\n...";
        let grid = input(&mut Cursor::new(input_str));
        assert_eq!(solve(&grid), 0);
    }

    fn test_all_rugs() {
        let input_str = "3 3\n+++\n+++\n+++";
        let grid = input(&mut Cursor::new(input_str));
        assert_eq!(solve(&grid), 1);
    }

    #[test]
    fn test_connected_rugs() {
        let input_str = "4 4
++++
....
++++
....";
        let grid = input(&mut Cursor::new(input_str));
        assert_eq!(solve(&grid), 2);
    }
}
