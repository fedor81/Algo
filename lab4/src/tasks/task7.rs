use lab1::modules::big_int::BigInt;
use num_bigint::BigUint;

use crate::modules::segtree_clone::SegTree;

pub fn run() {
    let (numbers, queries) = input();
    let results = solve_on_my_bigint(&numbers, &queries);

    for result in results {
        println!("{}", result);
    }
}

fn input() -> (Vec<u32>, Vec<Query>) {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let (length, queries_count) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );

    let mut arr = Vec::with_capacity(length);
    let mut queries = Vec::with_capacity(queries_count);

    for i in 0..length {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let n = buf.trim().parse().unwrap();
        arr.push(n);
    }

    for i in 0..queries_count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();

        queries.push(Query::from_tuple((
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )));
    }

    (arr, queries)
}

#[derive(Debug, Clone, Copy)]
pub enum Query {
    Sum { start: usize, end: usize },
    Replace { index: usize, element: u32 },
}

impl Query {
    fn from_tuple((type_, a, b): (u8, usize, usize)) -> Self {
        match type_ {
            1 => Query::Sum { start: a, end: b },
            2 => Query::Replace {
                index: a,
                element: b as u32,
            },
            _ => panic!("Invalid query type"),
        }
    }
}

pub fn solve_on_my_bigint(numbers: &[u32], queries: &[Query]) -> Vec<BigInt> {
    let mut results = Vec::with_capacity(queries.len());
    let mut segtree = SegTree::from_vec(
        &numbers
            .iter()
            .map(|&n| BigInt::from(n as i64))
            .collect::<Vec<BigInt>>(),
        |a, b| a + b,
    );

    for &query in queries {
        match query {
            Query::Replace { index, element } => {
                segtree.update(index, BigInt::from(element as i64)).unwrap()
            }
            Query::Sum { start, end } => results.push(segtree.query(start..=end)),
        }
    }

    results
}

pub fn solve_on_other_bigint(numbers: &[u32], queries: &[Query]) -> Vec<BigUint> {
    let mut results = Vec::with_capacity(queries.len());
    let mut segtree = SegTree::from_vec(
        &numbers
            .into_iter()
            .map(|&n| BigUint::from(n))
            .collect::<Vec<BigUint>>(),
        |a, b| a + b,
    );

    for &query in queries {
        match query {
            Query::Replace { index, element } => {
                segtree.update(index, BigUint::from(element)).unwrap()
            }
            Query::Sum { start, end } => results.push(segtree.query(start..=end)),
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};

    use super::*;

    #[test]
    fn tests_task7() {
        test_two_functions(
            &vec![1, 7, 15, 8, 9, 15, 15, 19, 5, 19],
            &vec![
                (1, 1, 8),
                (1, 6, 8),
                (1, 0, 6),
                (2, 6, 6),
                (2, 1, 6),
                (2, 0, 9),
                (1, 4, 7),
                (1, 3, 6),
            ],
            &vec![93, 39, 70, 49, 38],
        );
    }

    #[test]
    fn test_consecutive_replacements() {
        test_two_functions(
            &vec![1, 2, 3],
            &vec![
                (2, 0, 10), // Replace index 0 with 10
                (2, 0, 20), // Replace index 0 with 20
                (1, 0, 2),  // Sum query
            ],
            &vec![25], // 20 + 2 + 3 = 25
        );
    }

    #[test]
    fn test_range_queries() {
        test_two_functions(
            &vec![1, 2, 3, 4, 5],
            &vec![
                (1, 0, 4), // Sum of all elements
                (1, 1, 3), // Sum of middle elements
                (1, 0, 0), // Sum of first element
                (1, 4, 4), // Sum of last element
            ],
            &vec![15, 9, 1, 5],
        );
    }

    #[test]
    fn test_mixed_operations() {
        test_two_functions(
            &vec![1, 2, 3, 4],
            &vec![
                (1, 0, 3),  // Initial sum
                (2, 1, 10), // Replace index 1 with 10
                (1, 0, 3),  // Sum after replacement
                (2, 3, 5),  // Replace index 3 with 5
                (1, 0, 3),  // Final sum
            ],
            &vec![10, 18, 19], // 10, (1+10+3+4)=18, (1+10+3+5)=19
        );
    }

    #[test]
    #[should_panic(expected = "Invalid query type")]
    fn test_invalid_query_type() {
        let query = Query::from_tuple((3, 0, 0)); // 3 is invalid query type
    }

    fn test_two_functions(
        numbers: &Vec<u32>,
        queries: &Vec<(u8, usize, usize)>,
        expected: &Vec<u32>,
    ) {
        test_helper(solve_on_my_bigint, numbers, queries, expected);
        test_helper(solve_on_other_bigint, numbers, queries, expected);
    }

    fn test_helper<F, T>(f: F, numbers: &[u32], queries: &[(u8, usize, usize)], expected: &Vec<u32>)
    where
        F: Fn(&[u32], &[Query]) -> Vec<T>,
        T: Eq + Debug + Display,
    {
        let queries: Vec<_> = queries
            .iter()
            .map(|&tuple| Query::from_tuple(tuple))
            .collect();
        let actual: Vec<_> = f(numbers, &queries).iter().map(|n| n.to_string()).collect();
        let expected: Vec<_> = expected.iter().map(|n| n.to_string()).collect();
        assert_eq!(actual, expected);
    }
}
