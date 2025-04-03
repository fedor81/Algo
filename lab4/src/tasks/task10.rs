use std::collections::HashSet;

pub fn run() {
    let strings = input();
    let count = solve(&strings);
    println!("{}", count);
}

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut strings = Vec::new();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    for i in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        strings.push(buf.trim().to_string());
    }

    strings
}

pub fn solve(strings: &[String]) -> usize {
    let mut hash_set = HashSet::new();

    for string in strings {
        let mut letters = [0u16; 26];

        for &letter in string.as_bytes() {
            // letters[(letter - b'A') as usize] += 1;
            // Выполняется быстрее из за отсутствия проверки границ
            unsafe {
                *letters.get_unchecked_mut((letter - b'A') as usize) += 1;
            }
        }

        hash_set.insert(letters);
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task10() {
        test_helper(
            &vec!["BCB", "ABA", "BCB", "BAA", "BBC", "CCB", "CBC", "CBC"],
            3,
        );
    }

    #[test]
    fn test_single_string() {
        test_helper(&vec!["AAA"], 1);
    }

    #[test]
    fn test_same_pattern_different_letters() {
        test_helper(&vec!["AAA", "BBB", "CCC"], 3);
    }

    #[test]
    fn test_different_patterns() {
        test_helper(&vec!["AAE", "ABE", "ABC"], 3);
    }

    #[test]
    fn test_anagrams() {
        test_helper(&vec!["ABC", "CAB", "BCA", "CBA"], 1);
    }

    fn test_helper(strings: &Vec<&str>, expected: usize) {
        let strings: Vec<_> = strings.iter().map(|s| s.to_uppercase()).collect();
        let actual = solve(&strings);
        assert_eq!(actual, expected, "{:?}", strings);
    }
}
