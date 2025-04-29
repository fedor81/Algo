fn solve(digits_in_number: usize, numbering_system: usize) -> usize {
    match digits_in_number {
        // Первым не может идти 0
        2 => (numbering_system - 1) * numbering_system,
        1 => numbering_system,
        0 => 0,
        _ => {
            let mut numbers_not_ending_zero = numbering_system - 1;
            let mut numbers_ending_zero = 0;

            for _ in 2..=digits_in_number {
                (numbers_not_ending_zero, numbers_ending_zero) = (
                    (numbers_ending_zero + numbers_not_ending_zero) * (numbering_system - 1),
                    numbers_not_ending_zero * 1,
                );
            }

            numbers_not_ending_zero + numbers_ending_zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2, 10), 90);
        assert_eq!(solve(2, 3), 6);
        assert_eq!(solve(1, 3), 3);
        assert_eq!(solve(3, 10), 891);
        assert_eq!(solve(2, 10), 90);
    }

    #[test]
    fn test_binary_system() {
        // Testing with base 2 (binary)
        assert_eq!(solve(2, 2), 2); // In binary: 10, 11
        assert_eq!(solve(3, 2), 3); // In binary: 101, 110, 111
    }
}
