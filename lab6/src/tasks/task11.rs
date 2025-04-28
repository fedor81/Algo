pub fn run() {
    let (s1, s2) = input();
    let distance = levenshtein_distance(&s1, &s2);
    println!("{}", distance);
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let chars_a = a.chars().collect::<Vec<_>>();
    let chars_b = b.chars().collect::<Vec<_>>();

    let mut prev_row: Vec<usize> = (0..=chars_a.len()).collect();
    let mut curr_row = vec![0; chars_a.len() + 1];

    for i in 1..=chars_b.len() {
        curr_row[0] = i;

        for j in 1..=chars_a.len() {
            let add = prev_row[j] + 1;
            let del = curr_row[j - 1] + 1;
            let change = prev_row[j - 1] + (chars_a[j - 1] != chars_b[i - 1]) as usize;

            curr_row[j] = add.min(del).min(change);
        }

        prev_row.copy_from_slice(&curr_row);
    }

    curr_row[curr_row.len() - 1]
}

fn input() -> (String, String) {
    let stdin = std::io::stdin();
    let mut string1 = String::new();
    stdin.read_line(&mut string1).unwrap();
    let mut string2 = String::new();
    stdin.read_line(&mut string2).unwrap();

    (string1, string2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
        assert_eq!(levenshtein_distance("book", "back"), 2);
        assert_eq!(levenshtein_distance("abacaba", "abaabc"), 2);
        assert_eq!(levenshtein_distance("x", "r"), 1);
        assert_eq!(levenshtein_distance("innokentiy", "innokkentia"), 2);
        assert_eq!(levenshtein_distance("кот", "скат"), 2);
        assert_eq!(levenshtein_distance("rust", "rest"), 1);
    }
}
