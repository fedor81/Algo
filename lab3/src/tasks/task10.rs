pub fn run() {
    let input = input();
    let results = solve(&input.0, &input.1);

    for result in results {
        println!("{}", result + 1); // Увеличиваем индекс на еденицу, потому что по заданию нумерация начинаются с 1
    }
}

fn input() -> (Vec<Segment>, Vec<u32>) {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).unwrap();
    let count_segments = buf.trim().parse().unwrap();
    let mut segments = Vec::with_capacity(count_segments);

    for i in 0..count_segments {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let a = iter.next().unwrap().parse().unwrap();
        let b = iter.next().unwrap().parse().unwrap();
        segments.push(Segment::new(a, b));
    }

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let count_queries = buf.trim().parse().unwrap();
    let mut queries = Vec::with_capacity(count_queries);

    for i in 0..count_queries {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let query = buf.trim().parse().unwrap();
        queries.push(query);
    }

    (segments, queries)
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    left: u32,
    right: u32,
}

impl Segment {
    pub fn new(left: u32, right: u32) -> Self {
        Self { left, right }
    }

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn left(&self) -> u32 {
        self.left
    }
}

pub fn solve(segments: &Vec<Segment>, queries: &Vec<u32>) -> Vec<i32> {
    let mut results = Vec::with_capacity(queries.len());
    let previous_query = 0u16;

    for &query in queries {
        let mut found = false;
        let index = match find_nearest_right_bound(&segments, query) {
            None => segments.len(),
            Some(index) => index,
        };

        for i in (0..index).rev() {
            if segments[i].left <= query && query <= segments[i].right {
                results.push(i as i32);
                found = true;
                break;
            }
        }

        if !found {
            results.push(-1);
        }
    }

    results
}

/// Находит первую точку, у которой левая граница больше запроса.
/// Возвращает ее индекс
/// Использует бинарный поиск
fn find_nearest_right_bound(segments: &[Segment], query: u32) -> Option<usize> {
    let mut left = 0;
    let mut right = segments.len() - 1;

    while left < right {
        let mid = (left + right) / 2;

        if segments[mid].left <= query {
            left = mid + 1;
        } else if query < segments[mid].left {
            right = mid;
        }
    }

    if query < segments[left].left {
        Some(left)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SEGMENTS: [Segment; 9] = [
        Segment { left: 1, right: 7 },
        Segment { left: 2, right: 3 },
        Segment { left: 4, right: 6 },
        Segment { left: 5, right: 6 },
        Segment { left: 8, right: 14 },
        Segment { left: 9, right: 10 },
        Segment {
            left: 11,
            right: 14,
        },
        Segment {
            left: 12,
            right: 14,
        },
        Segment {
            left: 13,
            right: 14,
        },
    ];

    #[test]
    fn test_find_nearest_right_bound() {
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 0), Some(0));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 1), Some(1));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 2), Some(2));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 4), Some(3));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 5), Some(4));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 6), Some(4));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 8), Some(5));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 9), Some(6));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 12), Some(8));
        assert_eq!(find_nearest_right_bound(&SEGMENTS, 100), None);
    }

    #[test]
    fn test_task10() {
        assert_eq!(
            solve(
                &SEGMENTS.to_vec(),
                &vec![1, 2, 4, 5, 6, 7, 8, 8, 9, 12, 13, 14, 15, 100]
            ),
            vec![0, 1, 2, 3, 3, 0, 4, 4, 5, 7, 8, 8, -1, -1]
        );
    }

    #[test]
    fn test_single_segment() {
        let segments = vec![Segment::new(1, 5)];
        let queries = vec![1, 3, 5, 6];
        let results = solve(&segments, &queries);
        assert_eq!(results, vec![0, 0, 0, -1]);
    }

    #[test]
    fn test_consecutive_segments() {
        let segments = vec![Segment::new(1, 2), Segment::new(3, 4), Segment::new(5, 6)];
        let queries = vec![2, 3, 4, 5];
        let results = solve(&segments, &queries);
        assert_eq!(results, vec![0, 1, 1, 2]);
    }

    #[test]
    fn test_point_segments() {
        let segments = vec![Segment::new(1, 1), Segment::new(3, 3), Segment::new(5, 5)];
        let queries = vec![1, 2, 3, 4, 5];
        let results = solve(&segments, &queries);
        assert_eq!(results, vec![0, -1, 1, -1, 2]);
    }
}
