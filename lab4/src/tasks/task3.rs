use std::collections::HashSet;

pub fn run() {
    let sets = input();
    let count = find_largest_size_by_intersecting(&sets);
    println!("{}", count);
}

fn input() -> Vec<HashSet<i32>> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut sets = Vec::new();

    stdin.read_line(&mut buf).unwrap();
    let vec: Vec<_> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (count, length) = (vec[0], vec[1]);

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let set = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();
        sets.push(set);
    }

    sets
}

pub fn find_largest_size_by_intersecting(sets: &Vec<HashSet<i32>>) -> usize {
    let mut max_count = 0;

    for i in 0..(sets.len() - 1) {
        for j in (i + 1)..sets.len() {
            let a = &sets[i];
            let b = &sets[j];
            let count = a.intersection(b).count();

            if count > max_count {
                max_count = count;
            }
        }
    }

    max_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task3() {
        test_helper(
            &vec![vec![9, 7, 1, 8], vec![5, 7, 6, 3], vec![5, 9, 8, 6]],
            2,
        );
        test_helper(
            &vec![
                vec![-2, 6, 8, 4, -1],
                vec![5, 3, 10, -5, -1],
                vec![7, 8, -5, -1, -2],
                vec![-1, 8, 4, 9, 0],
            ],
            3,
        );
        test_helper(
            &vec![vec![1, 2, 3, 4], vec![3, 4, 5, 6], vec![4, 5, 6, 7]],
            3,
        );
    }

    fn test_helper(numbers: &Vec<Vec<i32>>, expected: usize) {
        let sets = numbers
            .iter()
            .map(|vec| vec.iter().cloned().collect())
            .collect();
        assert_eq!(find_largest_size_by_intersecting(&sets), expected);
    }

    #[test]
    fn test_no_intersection() {
        let sets = vec![
            vec![1, 2, 3].into_iter().collect(),
            vec![4, 5, 6].into_iter().collect(),
            vec![7, 8, 9].into_iter().collect(),
        ];
        assert_eq!(find_largest_size_by_intersecting(&sets), 0);
    }

    #[test]
    fn test_complete_intersection() {
        let sets = vec![
            vec![1, 2, 3].into_iter().collect(),
            vec![1, 2, 3].into_iter().collect(),
        ];
        assert_eq!(find_largest_size_by_intersecting(&sets), 3);
    }
}
