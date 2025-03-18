pub fn run() {
    let numbers = input();

    if is_min_heap(&numbers) {
        println!("YES")
    } else {
        println!("NO")
    }
}

fn input() -> Vec<i64> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let count = buf.trim().parse().unwrap();
    let mut numbers = Vec::new();

    for _i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        numbers.push(buf.trim().parse().unwrap());
    }

    numbers
}

fn is_min_heap(numbers: &Vec<i64>) -> bool {
    numbers.iter().enumerate().all(|(index, &value)| {
        let left = 2 * index + 1;
        let right = 2 * index + 2;

        (numbers.len() <= left || value <= numbers[left])
            && (numbers.len() <= right || value <= numbers[right])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task5() {
        // Basic cases
        test_helper(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9], true); // Valid min heap
        test_helper(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], false); // Invalid - 0 breaks heap property

        // Empty and single element heaps
        test_helper(&vec![], true); // Empty heap is valid
        test_helper(&vec![1], true); // Single element is valid

        // Small valid min heaps
        test_helper(&vec![1, 2, 3], true);
        test_helper(&vec![1, 4, 3, 7, 5], true);

        // Small invalid min heaps
        test_helper(&vec![3, 2, 1], false);
        test_helper(&vec![2, 1, 3], false);

        // Edge cases
        test_helper(&vec![-1, 0, 1], true); // Negative numbers
        test_helper(&vec![0, 1, 2], true); // Starting with zero

        // Larger test cases
        test_helper(&vec![1, 3, 2, 6, 5, 4, 7], true);
        test_helper(&vec![1, 2, 3, 4, 5, 6, 0], false);

        // Duplicate values
        test_helper(&vec![1, 1, 1], true); // All same values
        test_helper(&vec![1, 2, 2, 3, 3, 3], true); // Some duplicates
    }

    #[test]
    fn test_empty_heap() {
        assert!(is_min_heap(&vec![]));
    }

    #[test]
    fn test_single_element() {
        assert!(is_min_heap(&vec![42]));
    }

    #[test]
    fn test_invalid_heap_structure() {
        assert!(!is_min_heap(&vec![5, 4, 3, 2, 1]));
    }

    fn test_helper(numbers: &Vec<i64>, expected: bool) {
        assert_eq!(is_min_heap(numbers), expected);
    }
}
