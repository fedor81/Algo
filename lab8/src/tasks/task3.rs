use std::collections::HashMap;

pub fn run() {
    let mut text = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    println!("{}", solve(&text));
}

pub fn solve(text: &str) -> &str {
    let mut hash_map = HashMap::new();

    for start in 0..text.len() {
        for end in (start + 1)..=text.len() {
            let sub_string = &text[start..end];
            hash_map
                .entry(sub_string)
                .and_modify(|counter| *counter += 1)
                .or_insert(1usize);
        }
    }

    let (_, &max_frequency) = hash_map.iter().max_by_key(|(_, count)| *count).unwrap();
    hash_map
        .into_iter()
        .filter(|(_, count)| *count == max_frequency)
        .max_by_key(|(s, _)| s.len())
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("tebidohtebidoh"), "tebidoh");
    }

    #[test]
    fn test_aaaaaaaaaaa50() {
        assert_eq!(solve(&"a".repeat(50)), "a");
    }

    #[test]
    fn test_abababa() {
        assert_eq!(solve("abababa"), "a");
    }

    #[test]
    fn test_queue() {
        assert_eq!(solve("queue"), "ue");
    }

    #[test]
    fn test1() {
        assert_eq!(solve("abcabcbb"), "b");
    }

    #[test]
    fn test2() {
        assert_eq!(solve("bbbbb"), "b");
    }

    #[test]
    fn test3() {
        assert_eq!(solve("pwwkew"), "w");
    }
}
