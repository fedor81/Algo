use std::io::{BufRead, BufReader};

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let (s1, s2) = input(&mut reader);
    let result = find_shift(&s1, &s2);

    if let Some(result) = result {
        println!("{}", result);
    } else {
        println!("-1");
    }
}

/// Определяет число циклических сдвигов для получения строки T из S, либо выясняет,
/// что T не может быть получена из S.
pub fn find_shift(s: &str, t: &str) -> Option<usize> {
    let pattern = s.chars().collect::<Vec<_>>();
    let string = t.chars().chain(t.chars()).collect::<Vec<_>>();
    let length = string.len();
    let z = z_function(&pattern, &string);

    for i in 0..length {
        if z[i] >= pattern.len() {
            return Some(i);
        }
    }

    None
}

fn input<T: BufRead>(reader: &mut T) -> (String, String) {
    let mut s1 = String::new();
    let mut s2 = String::new();

    reader.read_line(&mut s1).unwrap();
    let len = s1.trim().parse::<usize>().unwrap();

    s1.clear();
    reader.read_line(&mut s1).unwrap();
    reader.read_line(&mut s2).unwrap();

    (s1.trim().to_string(), s2.trim().to_string())
}

pub fn z_function(pattern: &[char], string: &[char]) -> Vec<usize> {
    let mut z = vec![0; string.len()];

    // Значение функции для первого символа
    for i in 0..=string.len() {
        let letter = pattern.get(i);
        if letter.is_none() || letter != string.get(i) {
            z[0] = i;
            break;
        }
    }

    let mut l = 0;
    let mut r = 0;

    for i in 1..string.len() {
        // если мы уже видели этот символ
        if i <= r {
            // то мы можем попробовать его инициализировать z[i - l],
            // но не дальше правой границы: там мы уже ничего не знаем
            z[i] = (r - i + 1).min(z[i - l]);
        }

        // дальше каждое успешное увеличение z[i] сдвинет z-блок на единицу
        while i + z[i] < string.len() && pattern[z[i] % pattern.len()] == string[i + z[i]] {
            z[i] += 1;
        }

        // проверим, правее ли мы текущего z-блока
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

pub fn prefix_function(s: &str, t: &str) -> Vec<usize> {
    let s_chars = s.chars().collect::<Vec<_>>();
    let t_chars = t.chars().collect::<Vec<_>>();
    assert_eq!(s_chars.len(), t_chars.len(), "Strings must have equal length");
    let length = s_chars.len();

    // Считаем префикс-функцию
    let mut lps = vec![0; length];

    for i in 1..length {
        let mut k = lps[i - 1];

        while k > 0 && t_chars[i] != s_chars[k] {
            k = lps[k - 1];
        }

        if t_chars[i] == s_chars[k] {
            k += 1;
        }
        lps[i] = k;
    }
    lps
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{
        self,
        distr::{Alphanumeric, SampleString},
    };
    use std::io::Cursor;

    static COUNT: usize = 250000;

    #[test]
    fn test_with_input_abracadabra() {
        let s = "11
abracadabra
racadabraab
";
        let (s1, s2) = input(&mut Cursor::new(s));
        let actual = find_shift(&s1, &s2);

        assert_eq!(actual, Some(9));
    }

    #[test]
    fn test_abaaaa() {
        assert_eq!(find_shift("abaaaa", "aaaaba"), Some(3));
        assert_eq!(find_shift("abaaaa", "aaabaa"), Some(2));
        assert_eq!(find_shift("abaaaa", "baaaaa"), Some(5));
        assert_eq!(find_shift("abaaaa", "aaaaaa"), None);
    }

    #[test]
    fn test_no_possible_shift() {
        assert_eq!(find_shift("hello", "world"), None);
        assert_eq!(find_shift("abcde", "bcdea"), Some(4));
        assert_eq!(find_shift("abcde", "cdeab"), Some(3));
    }

    #[test]
    fn test_circular_shifts() {
        let test_cases = vec![
            ("abcde", "abcde", Some(0)),
            ("abcde", "eabcd", Some(1)),
            ("abcde", "deabc", Some(2)),
            ("abcde", "cdeab", Some(3)),
            ("abcde", "bcdea", Some(4)),
        ];

        for (s1, s2, expected) in test_cases {
            assert_eq!(find_shift(s1, s2), expected, "Failed for s1: {}, s2: {}", s1, s2);
        }
    }

    #[test]
    fn test_special_patterns() {
        // Test with repeating patterns
        assert_eq!(find_shift("aaaa", "aaaa"), Some(0));
        assert_eq!(find_shift("abab", "abab"), Some(0));
        assert_eq!(find_shift("abab", "baba"), Some(1));
    }

    #[test]
    // #[ignore]
    fn test_big_strings() {
        assert_eq!(find_shift(&"b".repeat(COUNT), &"a".repeat(COUNT)), None);
        assert_eq!(find_shift(&"ab".repeat(COUNT / 2), &"ba".repeat(COUNT / 2)), Some(1));
        assert_eq!(
            find_shift(
                &format!("{}a", "b".repeat(COUNT - 1)),
                &format!("a{}", "b".repeat(COUNT - 1))
            ),
            Some(1)
        );
        assert_eq!(
            find_shift(
                &format!("a{}", "b".repeat(COUNT - 1)),
                &format!("{}a", "b".repeat(COUNT - 1))
            ),
            Some(COUNT - 1)
        );
    }

    #[test]
    // #[ignore]
    fn test_random_strings() {
        let s = Alphanumeric.sample_string(&mut rand::rng(), COUNT);
        let shift = rand::random_range(1..COUNT);
        let t = format!("{}{}", &s[shift..], &s[..shift]);
        assert_eq!(find_shift(&s, &t), Some(COUNT - shift));
    }

    #[test]
    fn test_prefix_function() {
        test_prefix_function_helper("abracadabra", "abaracardar", &vec![0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0]);
        test_prefix_function_helper("abracadabra", "abracadabra", &vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_z_function() {
        test_z_function_helper("abracadabra", "abaracardar", &vec![2, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0]);
        test_z_function_helper("abracadabra", "abracadabra", &vec![11, 0, 0, 1, 0, 1, 0, 4, 0, 0, 1]);
        test_z_function_helper("abaaba", "abaaba", &vec![6, 0, 1, 3, 0, 1]);
        test_z_function_helper("aba", "abaaba", &vec![3, 0, 1, 3, 0, 1]);
    }

    fn test_z_function_helper(s: &str, t: &str, expected: &[usize]) {
        let z = z_function(&s.chars().collect::<Vec<_>>(), &t.chars().collect::<Vec<_>>());

        assert_eq!(
            z,
            expected,
            "\nS:\t[{}]\nT:\t[{}]",
            s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
            t.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
        );
    }

    fn test_prefix_function_helper(s: &str, t: &str, expected: &[usize]) {
        let lps = prefix_function(s, t);

        assert_eq!(
            lps,
            expected,
            "\nS:\t[{}]\nT:\t[{}]",
            s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
            t.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(", "),
        );
    }
}
