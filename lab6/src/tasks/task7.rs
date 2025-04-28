use lab3::modules::heap_sort::{heap_sort, heap_sort_cmp};

pub fn run() {
    let presentations = input();
    let count = get_attended_interesting_presentations(presentations);
}

fn get_attended_interesting_presentations(mut presentations: Vec<Presentation>) -> usize {
    heap_sort(&mut presentations, false);
    let mut count_attended = 0;
    let mut current_presentation_end = 0;

    for presentation in presentations {
        if current_presentation_end + 1 <= presentation.start {
            count_attended += 1;
            current_presentation_end = presentation.end;
        }
    }

    count_attended
}

fn input() -> Vec<Presentation> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();
    let mut presentations = vec![];

    for i in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        presentations.push(Presentation::from_str(&buf));
    }

    presentations
}

#[derive(Debug, Clone, Copy, Ord, PartialEq, Eq)]
struct Presentation {
    start: u16,
    end: u16,
}

impl Presentation {
    fn new(start: u16, end: u16) -> Self {
        Self { start, end }
    }

    pub fn from_str(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();

        Self::new(start, end)
    }
}

impl PartialOrd for Presentation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.end.partial_cmp(&other.end) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.start.partial_cmp(&other.start)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_sort() {
        let expected = vec![
            Presentation::new(1, 2),
            Presentation::new(1, 3),
            Presentation::new(2, 3),
            Presentation::new(3, 4),
            Presentation::new(1, 5),
            Presentation::new(3, 5),
            Presentation::new(4, 5),
        ];
        let mut input = vec![
            Presentation::new(2, 3),
            Presentation::new(1, 5),
            Presentation::new(4, 5),
            Presentation::new(1, 2),
            Presentation::new(3, 5),
            Presentation::new(1, 3),
            Presentation::new(3, 4),
        ];

        heap_sort(&mut input, false);
        assert_eq!(input, expected)
    }

    #[test]
    fn test_task7() {
        test_helper(vec![(3, 4), (1, 5), (6, 7), (4, 5), (1, 3)], 3);
    }

    #[test]
    fn test_complex_scenario() {
        test_helper(
            vec![
                (1, 2), // Take this one
                (1, 3),
                (2, 3),
                (3, 4), // Take this one
                (1, 5),
                (5, 6), // Take this one
                (7, 8), // Take this one
            ],
            4,
        );
    }

    #[test]
    fn test_non_overlapping_presentations() {
        test_helper(vec![(1, 2), (3, 4), (5, 6)], 3);
    }

    #[test]
    fn test_completely_overlapping_presentations() {
        test_helper(vec![(1, 5), (2, 3), (2, 4), (1, 6)], 1);
    }

    #[test]
    fn test_same_start_different_end() {
        test_helper(vec![(2, 5), (2, 3), (2, 4)], 1);
    }

    #[test]
    fn test_same_end_different_start() {
        test_helper(vec![(1, 5), (2, 5), (3, 5)], 1);
    }

    #[test]
    fn test_partially_overlapping_presentations() {
        test_helper(vec![(1, 3), (2, 4), (3, 5)], 1);
        test_helper(vec![(1, 3), (2, 4), (4, 5)], 2);
    }

    fn test_helper(presentations: Vec<(u16, u16)>, expected: usize) {
        let presentations = presentations
            .into_iter()
            .map(|(start, end)| Presentation::new(start, end))
            .collect();

        let actual = get_attended_interesting_presentations(presentations);
        assert_eq!(actual, expected);
    }
}
