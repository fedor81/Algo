pub fn run() {
    let (input, pattern) = input();
    let result = match_pattern(&input, &pattern);

    if result {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn input() -> (String, String) {
    let stdin = std::io::stdin();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut pattern = String::new();
    stdin.read_line(&mut pattern).unwrap();

    (input.trim().to_string(), pattern.trim().to_string())
}

fn match_pattern(input: &str, pattern: &str) -> bool {
    let input_chars: Vec<char> = input.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    match_pattern_helper(&input_chars, &pattern_chars)
}

fn match_pattern_helper(input: &[char], pattern: &[char]) -> bool {
    match (input.is_empty(), pattern.is_empty()) {
        (true, true) => true,
        (true, false) => pattern.iter().all(|&c| c == '*'),
        (false, true) => false,
        _ => {
            let (s_first, s_rest) = (&input[0], &input[1..]);
            let (p_first, p_rest) = (&pattern[0], &pattern[1..]);

            match p_first {
                '?' => match_pattern_helper(s_rest, p_rest),
                '*' => match_pattern_helper(input, p_rest) || match_pattern_helper(s_rest, pattern),
                _ => {
                    if s_first == p_first {
                        match_pattern_helper(s_rest, p_rest)
                    } else {
                        false
                    }
                }
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_basic_matching() {
        assert_eq!(match_pattern("abacaba", "abacaba"), true);
        assert_eq!(match_pattern("hello", "hello"), true);
        assert_eq!(match_pattern("", ""), true);
    }

    #[test]
    fn test_basic_non_matching() {
        assert_eq!(match_pattern("abacaba", "abacab"), false);
        assert_eq!(match_pattern("hello", "world"), false);
        assert_eq!(match_pattern("abc", "def"), false);
    }

    #[test]
    fn test_question_mark_pattern() {
        assert_eq!(match_pattern("U", "?"), true);
        assert_eq!(match_pattern("hello", "h?llo"), true);
        assert_eq!(match_pattern("hello", "h??lo"), true);
        assert_eq!(match_pattern("", "?"), false);
        assert_eq!(match_pattern("a", "??"), false);
    }

    #[test]
    fn test_asterisk_pattern() {
        assert_eq!(match_pattern("a", "*"), true);
        assert_eq!(match_pattern("aaaa", "*"), true);
        assert_eq!(match_pattern("", "*"), true);
        assert_eq!(match_pattern("hello", "h*o"), true);
        assert_eq!(match_pattern("hello", "*o"), true);
        assert_eq!(match_pattern("hello", "h*"), true);
    }

    #[test]
    fn test_complex_patterns() {
        assert_eq!(match_pattern("aaaa", "?*?"), true);
        assert_eq!(match_pattern("aaaa", "a?*?a"), true);
        assert_eq!(match_pattern("ABRACADABRA", "ABRA*ABRA"), true);
        assert_eq!(match_pattern("FOOBAR", "F*??O*"), false);
        assert_eq!(match_pattern("testGGGtest", "test*test"), true);
        assert_eq!(match_pattern("aabbcc", "a*b?c*"), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(match_pattern("", "*"), true);
        assert_eq!(match_pattern("", ""), true);
        assert_eq!(match_pattern("a", ""), false);
        assert_eq!(match_pattern("", "a"), false);
        assert_eq!(match_pattern("abc", "***"), true);
        assert_eq!(match_pattern("abc", "????"), false);
    }

    #[test]
    fn test_long_pattern() {
        assert_eq!(match_pattern(&"A".repeat(700), &"A".repeat(700)), true);
        assert_eq!(match_pattern(&"A".repeat(700), &"*".repeat(700)), true);
        assert_eq!(match_pattern(&"A".repeat(700), &"?".repeat(700)), true);
        assert_eq!(match_pattern(&"A".repeat(700), &"?*".repeat(350)), true);
        assert_eq!(match_pattern(&"A".repeat(700), &"A*".repeat(350)), true);
    }
}
