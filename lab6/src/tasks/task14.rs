use std::ops::{Index, IndexMut};

pub fn run() {
    let map = input();
    let (way, cost) = find_cheap_way(&map);

    println!("");

    for i in way {
        print!("{} ", i + 1);
    }

    println!("");
    println!("{}", cost);
}

fn find_cheap_way(map: &Map) -> (Vec<usize>, i32) {
    let rows = map.rows();
    let cols = map.cols();

    let mut best_way = None;
    let mut min_cost = i32::MAX;

    let mut curr_row = vec![0; cols];
    let mut prev_row = vec![0; cols];
    let mut way_parents = vec![vec![None; rows]; cols];

    // Заполнение первого/стартового ряда
    for col in 0..cols {
        prev_row[col] = map[col][0];
    }

    // Заполнение последующих рядов
    for row in 1..rows {
        for col in 0..cols {
            let mut min_parent = None;
            let mut min_parent_cost = i32::MAX;

            for (i, parent_cost) in get_neighbors(&prev_row, col) {
                if parent_cost < min_parent_cost {
                    min_parent = Some(i);
                    min_parent_cost = parent_cost;
                }
            }

            curr_row[col] = map[col][row] + min_parent_cost;
            way_parents[col][row] = min_parent;
        }
        (prev_row, curr_row) = (curr_row, prev_row);
    }

    // Выбор пути и стоимости
    for col in 0..cols {
        let cost = prev_row[col];

        if cost < min_cost {
            min_cost = cost;
            best_way = Some(get_way_parents(&way_parents, col))
        } else if cost == min_cost {
            let new_way = get_way_parents(&way_parents, col);

            match best_way.as_ref() {
                Some(current_best) => {
                    if &new_way < current_best {
                        best_way = Some(new_way);
                    }
                }
                None => best_way = Some(new_way),
            }
        }
    }

    (best_way.unwrap(), min_cost)
}

fn get_way_parents(way_parents: &Vec<Vec<Option<usize>>>, mut col: usize) -> Vec<usize> {
    let mut parents = vec![col];
    let mut row = way_parents[0].len() - 1;

    while let Some(parent) = way_parents[col][row] {
        parents.push(parent);
        col = parent;
        row -= 1;
    }

    parents.reverse();
    parents
}

fn get_neighbors<T: Copy>(v: &Vec<T>, index: usize) -> impl Iterator<Item = (usize, T)> {
    (-1..=1).filter_map(move |offset| {
        let i = index as i32 + offset;
        if i >= 0 && i < v.len() as i32 {
            let i = i as usize;
            Some((i, v[i]))
        } else {
            None
        }
    })
}

type Column = Vec<i32>;

/// `Map[col][row]` индексация
#[derive(Debug)]
struct Map {
    map: Vec<Column>,
}

impl Index<usize> for Map {
    type Output = Column;

    fn index(&self, index: usize) -> &Self::Output {
        &self.map[index]
    }
}

impl IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.map[index]
    }
}

impl Map {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            map: vec![vec![0; rows]; cols],
        }
    }

    pub fn from_vec(map: Vec<Column>) -> Self {
        Self { map }
    }

    pub fn rows(&self) -> usize {
        self.map[0].len()
    }

    pub fn cols(&self) -> usize {
        self.map.len()
    }
}

fn input() -> Map {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let (cols, rows) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );

    let mut map = Map::new(rows, cols);

    for col in 0..cols {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut iter = buf
            .split_whitespace()
            .into_iter()
            .map(|s| s.parse().unwrap());

        for row in 0..rows {
            map[col][row] = iter.next().unwrap();
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_task() {
        let map = Map::from_vec(vec![
            vec![1, 7, 4, 3],
            vec![5, 1, 6, 7],
            vec![4, 1, 9, 2],
            vec![7, 3, 7, 5],
            vec![8, 2, 4, 1],
        ]);

        test_helper(map, vec![1, 2, 1, 1], 9);
    }

    #[test]
    fn test_from_task_v2() {
        let map = Map::from_vec(vec![
            vec![3, 4, 6, 2, 8, 6],
            vec![6, 1, 8, 2, 7, 4],
            vec![5, 9, 3, 9, 9, 5],
            vec![8, 4, 1, 3, 9, 6],
            vec![3, 7, 2, 8, 6, 4],
        ]);

        test_helper(map, vec![1, 2, 3, 2, 2, 2], 20);
    }

    fn test_helper(map: Map, expected_way: Vec<usize>, expected_cost: i32) {
        let (mut way, cost) = find_cheap_way(&map);
        way.iter_mut().for_each(|i| *i += 1);

        assert_eq!(cost, expected_cost);
        assert_eq!(way, expected_way);
    }

    #[test]
    fn get_way_parents_test() {
        let way_parents = vec![
            vec![None, Some(1), Some(1), Some(0)],
            vec![None, Some(0), Some(0), Some(2)],
            vec![None, Some(2), Some(1), Some(1)],
            vec![None, Some(0), Some(2), Some(4)],
            vec![None, Some(4), Some(4), Some(3)],
        ];

        assert_eq!(get_way_parents(&way_parents, 0), vec![0, 1, 0, 0]);
        assert_eq!(get_way_parents(&way_parents, 1), vec![0, 1, 2, 1]);
        assert_eq!(get_way_parents(&way_parents, 2), vec![1, 0, 1, 2]);
        assert_eq!(get_way_parents(&way_parents, 3), vec![4, 4, 4, 3]);
        assert_eq!(get_way_parents(&way_parents, 4), vec![2, 2, 3, 4]);
    }

    #[test]
    fn get_neighbors_test() {
        let vector = vec![1, 2, 3, 4, 5];

        assert_eq!(
            get_neighbors(&vector, 2).collect::<Vec<_>>(),
            vec![(1, 2), (2, 3), (3, 4)]
        );
        assert_eq!(
            get_neighbors(&vector, 0).collect::<Vec<_>>(),
            vec![(0, 1), (1, 2)]
        );
        assert_eq!(
            get_neighbors(&vector, 4).collect::<Vec<_>>(),
            vec![(3, 4), (4, 5)]
        );
    }

    #[test]
    fn test_get_neighbors_empty_vec() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(get_neighbors(&empty_vec, 0).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn test_get_neighbors_single_element() {
        let vec = vec![1];
        assert_eq!(get_neighbors(&vec, 0).collect::<Vec<_>>(), vec![(0, 1)]);
    }

    #[test]
    fn map_test() {
        let map = Map::new(3, 4);

        assert_eq!(map.rows(), 3);
        assert_eq!(map.cols(), 4);

        let map = Map::from_vec(vec![
            vec![1, 7, 4, 3],
            vec![5, 1, 6, 7],
            vec![4, 1, 9, 2],
            vec![7, 3, 7, 5],
            vec![8, 2, 4, 1],
        ]);

        assert_eq!(map.cols(), 5);
        assert_eq!(map.rows(), 4);

        assert_eq!(map[0][0], 1);
        assert_eq!(map[0][1], 7);
        assert_eq!(map[0][2], 4);
    }

    #[test]
    fn test_single_column() {
        let map = Map::from_vec(vec![vec![1, 2, 3]]);
        let (way, cost) = find_cheap_way(&map);
        assert_eq!(cost, 6);
        assert_eq!(way, vec![0, 0, 0]);
    }

    #[test]
    fn test_single_row() {
        let map = Map::from_vec(vec![vec![1], vec![2], vec![3]]);
        let (way, cost) = find_cheap_way(&map);
        assert_eq!(cost, 1);
        assert_eq!(way, vec![0]);
    }

    #[test]
    fn test_equal_paths() {
        let map = Map::from_vec(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]);
        let (way, cost) = find_cheap_way(&map);
        assert_eq!(cost, 3);
        // Should choose leftmost path when equal
        assert_eq!(way, vec![0, 0, 0]);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn test_invalid_column_access() {
        let map = Map::new(3, 3);
        let _value = map[3][0]; // Should panic
    }

    #[test]
    fn test_negative_values() {
        let map = Map::from_vec(vec![vec![-1, -2, -3], vec![-2, -4, -1], vec![-3, -2, -4]]);
        let (way, cost) = find_cheap_way(&map);
        assert_eq!(cost, -11); // Should find the path with minimum sum
        assert_eq!(way, vec![2, 1, 2]);
    }

    #[test]
    fn test_zigzag_pattern() {
        let map = Map::from_vec(vec![vec![9, 9, 9, 9], vec![1, 1, 1, 9], vec![9, 1, 1, 1]]);
        let (way, cost) = find_cheap_way(&map);
        assert_eq!(cost, 4);
        assert_eq!(way, vec![1, 1, 1, 2]);
    }

    #[test]
    fn test_map_mutation() {
        let mut map = Map::new(2, 2);
        map[0][0] = 5;
        map[0][1] = 6;
        map[1][0] = 7;
        map[1][1] = 8;

        assert_eq!(map[0][0], 5);
        assert_eq!(map[1][1], 8);
    }
}
