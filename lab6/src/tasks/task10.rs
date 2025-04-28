//! Задание не решено!

use lab3::modules::heap_sort::heap_sort_cmp;

pub fn run() {
    let input = input();
    let result = solve(input);

    println!("{}", result.time);

    for sheet in result.left.into_iter().chain(result.right.into_iter()) {
        print!("{} ", sheet);
    }
}

fn solve(mut dissolutions: Vec<DissolutionTime>) -> SolveResult {
    let mut left_time = 0f64;
    let mut right_time = 0f64;
    let mut left_sheets = vec![];
    let mut right_sheets = vec![];

    heap_sort_cmp(&mut dissolutions, |&a, &b| {
        a.left.min(a.right) > b.left.min(b.right)
    });

    for dissolution in dissolutions {
        if dissolution.left > dissolution.right {
            left_time += dissolution.left;
            left_sheets.push(dissolution.index);
        } else {
            right_time += dissolution.right;
            right_sheets.push(dissolution.index);
        }
    }

    SolveResult::new(left_time.min(right_time), left_sheets, right_sheets)
}

#[derive(Debug, Clone, Copy)]
struct DissolutionTime {
    left: f64,
    right: f64,
    index: usize,
}

struct SolveResult {
    time: f64,
    left: Vec<usize>,
    right: Vec<usize>,
}

impl SolveResult {
    pub fn new(time: f64, left: Vec<usize>, right: Vec<usize>) -> Self {
        Self { time, left, right }
    }
}

fn input() -> Vec<DissolutionTime> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let count_sheets = buf.trim().parse().unwrap();
    let mut dissolution_times = vec![];

    for i in 0..count_sheets {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        dissolution_times.push(DissolutionTime::from_str(&buf, i + 1));
    }

    dissolution_times
}
impl DissolutionTime {
    pub fn from_str(s: &str, index: usize) -> Self {
        let mut parts = s.split_whitespace();
        let left = parts.next().unwrap().parse().unwrap();
        let right = parts.next().unwrap().parse().unwrap();

        Self { left, right, index }
    }

    pub fn new(left: f64, right: f64, index: usize) -> Self {
        Self { left, right, index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_from_condition() {
        test_helper(
            vec![(1.0, 2.0), (1.0, 2.0), (0.5, 1.5), (7.0, 3.5)],
            6.0,
            vec![4],
            vec![2, 1, 3],
        );
    }

    #[test]
    fn test_10_vs_1_2_3_4() {
        test_helper_u32(
            vec![(1, 2), (10, 1), (1, 3), (1, 4), (1, 1)],
            10,
            vec![2],
            vec![5, 1, 3, 4],
        );
    }

    fn test_helper_u32(
        times: Vec<(u32, u32)>,
        expected_time: u32,
        expected_left: Vec<usize>,
        expected_right: Vec<usize>,
    ) {
        let times = times
            .into_iter()
            .map(|(left, right)| (left as f64, right as f64))
            .collect();
        test_helper(times, expected_time as f64, expected_left, expected_right);
    }

    fn test_helper(
        times: Vec<(f64, f64)>,
        expected_time: f64,
        mut expected_left: Vec<usize>,
        mut expected_right: Vec<usize>,
    ) {
        let times = times
            .into_iter()
            .enumerate()
            .map(|(i, (left, right))| DissolutionTime::new(left, right, i + 1))
            .collect();
        let mut actual = solve(times);

        actual.left.sort();
        actual.right.sort();
        expected_left.sort();
        expected_right.sort();

        assert_eq!(actual.time, expected_time);
        assert_eq!(actual.left, expected_left);
        assert_eq!(actual.right, expected_right);
    }
}
