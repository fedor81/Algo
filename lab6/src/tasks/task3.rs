use std::i64;

use lab3::modules::heap_sort;

pub fn run() {
    let (max_dance_distance, points) = input();
    let min_count = get_remaining_points(max_dance_distance, points);
    println!("{}", min_count);
}

fn input() -> (u32, Vec<i64>) {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let max_dance_distance = iter.next().unwrap().parse().unwrap();
    let _count: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let points: Vec<_> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    (max_dance_distance, points)
}

pub fn get_remaining_points(max_distance: u32, mut points: Vec<i64>) -> usize {
    heap_sort::heap_sort(&mut points, false);

    let max_distance = max_distance as i64;
    let mut result = 1;
    let mut current_dance_distance = points[0] + max_distance;

    for point in points {
        if current_dance_distance < point - max_distance {
            result += 1;
            current_dance_distance = point + max_distance;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task3() {
        assert_eq!(get_remaining_points(10, vec![30, 3, 14, 19, 21]), 2);
        assert_eq!(get_remaining_points(10, vec![-30, -3, -14, -19, -21]), 2);
        assert_eq!(get_remaining_points(5, vec![0, 4, 9, 14]), 2);
        assert_eq!(get_remaining_points(10, vec![-10, 10]), 1);
    }

    #[test]
    fn test_single_point() {
        assert_eq!(get_remaining_points(10, vec![1]), 1);
    }

    #[test]
    fn test_points_within_range() {
        assert_eq!(get_remaining_points(10, vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(get_remaining_points(2, vec![1, 2, 3, 4, 5]), 1);
    }

    #[test]
    fn test_points_at_exact_distance() {
        // Points exactly max_distance apart
        assert_eq!(get_remaining_points(4, vec![1, 6, 11, 16]), 2);
        assert_eq!(get_remaining_points(5, vec![1, 7, 13, 19, 25]), 3);
    }

    #[test]
    fn test_unordered_points() {
        // Test that unordered input works correctly
        assert_eq!(get_remaining_points(10, vec![25, 5, 15, 35]), 2);
    }

    #[test]
    fn test_zero_max_distance() {
        // Each point needs its own coverage
        assert_eq!(get_remaining_points(0, vec![1, 2, 3]), 3);
    }

    #[test]
    fn test_duplicate_points() {
        assert_eq!(get_remaining_points(10, vec![5, 5, 5, 5]), 1);
    }

    #[test]
    fn big_test() {
        let count = 10000;
        let max_distance = 100000000;

        assert_eq!(
            get_remaining_points(
                max_distance,
                ((-count / 2)..(count / 2))
                    .into_iter()
                    .map(|i| i * 200000)
                    .collect()
            ),
            10
        );
    }
}
