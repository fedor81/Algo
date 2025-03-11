use crate::modules::sorting::bubble_sort;
use std::collections::HashSet;

pub fn run() {
    let (set1, set2) = input();
    let result_set = symmetrical_difference(set1, set2);

    if result_set.len() == 0 {
        println!("0");
        return;
    }

    let sorted_result = bubble_sort(result_set);

    for num in sorted_result {
        print!("{num} ");
    }
    println!();
}

fn input() -> (HashSet<i32>, HashSet<i32>) {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin
        .read_line(&mut input)
        .expect("Не получается прочитать строку");

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut add_to_first_set = true;

    for s in input.split_whitespace() {
        let number: i32 = s.parse().expect(&format!("Не удалось запарсить {}", s));

        if number == 0 {
            add_to_first_set = false;
        } else if add_to_first_set {
            set1.insert(number);
        } else {
            set2.insert(number);
        }
    }

    (set1, set2)
}

fn symmetrical_difference(set1: HashSet<i32>, set2: HashSet<i32>) -> HashSet<i32> {
    let mut result = HashSet::new();

    for num in set1.union(&set2) {
        if set1.contains(&num) ^ set2.contains(&num) {
            result.insert(*num);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetrical_difference_1() {
        let result =
            symmetrical_difference(HashSet::from([3, 4, 5, 6]), HashSet::from([1, 2, 3, 4]));
        assert_eq!(result, HashSet::from([1, 2, 5, 6]));
    }

    #[test]
    fn test_symmetrical_difference_2() {
        let result =
            symmetrical_difference(HashSet::from([1, 2, 3, 4, 5]), HashSet::from([1, 2, 3, 4]));
        assert_eq!(result, HashSet::from([5]));
    }

    #[test]
    fn test_symmetrical_difference_3() {
        let result = symmetrical_difference(
            HashSet::from([1, 2, 3, 4, 5]),
            HashSet::from([1, 2, 3, 4, 5]),
        );
        assert_eq!(result, HashSet::from([]));
    }

    #[test]
    fn test_symmetrical_difference_4() {
        let result =
            symmetrical_difference(HashSet::from([1, 2, 3]), HashSet::from([2, 3, 4, 5, 9999]));
        assert_eq!(result, HashSet::from([1, 4, 5, 9999]));
    }
}
