use std::fmt::Display;

pub fn run() {
    let mut grid = input();
    solve(&mut grid);

    println!("{}", grid.operations().len());

    for operation in grid.operations() {
        println!("{}", operation);
    }
}

fn input() -> Grid {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin
        .read_line(&mut buf)
        .expect("Не удалось прочитать строку с консоли");

    let size: Vec<_> = buf
        .split_whitespace()
        .map(|i| i.parse().expect("Не удалось спарсить число"))
        .collect();

    let mut grid = Grid::new(size[0], size[1]);

    for i in 0..size[0] {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        for (j, number) in buf.split_whitespace().enumerate() {
            let number = number.parse().unwrap();
            grid.set(i, j, number);
        }
    }

    grid
}

fn solve(grid: &mut Grid) {
    for i in 0..grid.rows() {
        for j in 0..grid.columns() {
            let expected = (i * grid.columns() + j + 1) as u16;

            loop {
                let current = grid.get(i, j);

                if current == expected {
                    break;
                }

                let row_needed = (current - 1) / (grid.rows() as u16);
                let col_needed = (current - 1) % (grid.columns() as u16);

                if row_needed != i as u16 {
                    grid.swap_rows(i, row_needed.into());
                }

                if col_needed != j as u16 {
                    grid.swap_columns(j, col_needed.into());
                }
            }
        }
    }
}

struct Grid {
    vector: Vec<Box<Vec<u16>>>,
    operations: Vec<Operation>,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        if !(1 <= rows && rows <= 250 && 1 <= columns && columns <= 250) {
            panic!("Invalid grid size");
        }

        Grid {
            vector: vec![Box::from(vec![0; columns]); rows],
            operations: vec![],
        }
    }

    pub fn rows(&self) -> usize {
        self.vector.len()
    }

    pub fn columns(&self) -> usize {
        self.vector[0].len()
    }

    fn check(&self, row: usize, column: usize) {
        if !(row < self.vector.len() && column < self.vector[0].len()) {
            panic!("Invalid grid index");
        }
    }

    pub fn get(&self, row: usize, column: usize) -> u16 {
        self.check(row, column);
        self.vector[row][column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: u16) {
        self.check(row, column);
        self.vector[row][column] = value;
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        (self.vector[row1], self.vector[row2]) =
            (self.vector[row2].clone(), self.vector[row1].clone());
        self.operations
            .push(Operation::new(Direction::Row, row1 as u16, row2 as u16));
    }

    pub fn swap_columns(&mut self, column1: usize, column2: usize) {
        for i in 0..self.vector.len() {
            (self.vector[i][column1], self.vector[i][column2]) =
                (self.vector[i][column2], self.vector[i][column1]);
        }
        self.operations.push(Operation::new(
            Direction::Column,
            column1 as u16,
            column2 as u16,
        ));
    }

    pub fn operations(&self) -> &Vec<Operation> {
        &self.operations
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.vector {
            for number in row.iter() {
                f.write_fmt(format_args!("{} ", number))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Row,
    Column,
}

#[derive(Clone)]
struct Operation {
    direction: Direction,
    index1: u16,
    index2: u16,
}

impl Operation {
    pub fn new(direction: Direction, index1: u16, index2: u16) -> Self {
        Operation {
            direction,
            index1,
            index2,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            if self.direction == Direction::Row {
                "R"
            } else {
                "C"
            },
            // В отображении увеливаем индексы на еденицу
            self.index1 + 1,
            self.index2 + 1
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_task10() {
        test_helper(2, 2, vec!["4 3", "2 1"]);
        test_helper(3, 5, vec!["1 2 3 4 5", "6 7 8 9 10", "11 12 13 14 15"]);
        test_helper(
            4,
            5,
            vec![
                "10 7 9 8 6",
                "15 12 14 13 11",
                "20 17 19 18 16",
                "5 2 4 3 1",
            ],
        );
    }

    #[test]
    fn random_test() {
        let size = 250;
        let mut rng = rand::rng();

        for test_number in 0..30 {
            let mut grid = get_squared_grid(size);

            for _ in 0..500 {
                let n1 = rng.random_range(0..size);
                let n2 = rng.random_range(0..size);

                if n1 != n2 && rng.random_bool(0.5) {
                    grid.swap_columns(n1, n2);
                } else {
                    grid.swap_rows(n1, n2);
                }
            }

            solve(&mut grid);
            assert!(grid.operations().len() <= 1000);

            for i in 0..size {
                for j in 0..size {
                    let expected = (i * size + j + 1) as u16;
                    assert_eq!(grid.get(i as usize, j as usize), expected);
                }
            }
        }
    }

    fn get_squared_grid(size: usize) -> Grid {
        let mut grid = Grid::new(size, size);

        for i in 0..size {
            for j in 0..size {
                let value = (i * size + j + 1) as u16;
                grid.set(i, j, value);
            }
        }

        grid.operations.clear();
        grid
    }

    fn test_helper(rows: u16, cols: u16, input: Vec<&str>) {
        let mut grid = Grid::new(rows as usize, cols as usize);

        for i in 0..input.len() {
            for (j, number) in input[i].split_whitespace().enumerate() {
                let number = number.parse().unwrap();
                grid.set(i, j, number);
            }
        }

        solve(&mut grid);
        assert!(grid.operations().len() <= 1000);

        for i in 0..rows {
            for j in 0..cols {
                let expected = (i * cols + j + 1) as u16;
                assert_eq!(grid.get(i as usize, j as usize), expected);
            }
        }
    }
}
