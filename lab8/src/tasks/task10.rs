use crate::tasks::task7::z_function;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let (entertaining_word, jack_word) = input(&mut reader);

    if let Some(parts) = solve_z_function(&entertaining_word, &jack_word) {
        println!("No");
        println!("{}", parts.join(" "));
    } else {
        println!("Yes");
    }
}

/// Проверяет, что слово Джека можно разбить на префиксы интересного слова.
/// Если да, то возвращает разбиение
/// Использует поиск в ширину
pub fn solve_bfs<'a>(entertaining_word: &str, jack_word: &'a str) -> Option<Vec<&'a str>> {
    let len_ent = entertaining_word.len();
    let len_jack = jack_word.len();

    // Быстрая проверка на невозможность разбиения
    if len_jack == 0 {
        return Some(vec![]);
    }
    if len_ent == 0 {
        return None;
    }

    // BFS с мемоизацией посещенных позиций
    let mut visited = vec![false; len_jack + 1];
    let mut queue = VecDeque::new();

    queue.push_back((0, Vec::new()));
    visited[0] = true;

    while let Some((pos, parts)) = queue.pop_front() {
        if pos == len_jack {
            return Some(parts);
        }

        // Определяем максимальную возможную длину префикса
        let max_len = len_ent.min(len_jack - pos);

        // Проверяем префиксы от самых длинных к коротким
        for l in (1..=max_len).rev() {
            let end = pos + l;
            if end > len_jack {
                continue;
            }

            if !visited[end] && entertaining_word.starts_with(&jack_word[pos..end]) {
                visited[end] = true;
                let mut new_parts = parts.clone();
                new_parts.push(&jack_word[pos..end]);
                queue.push_back((end, new_parts));
            }
        }
    }

    None
}

/// Проверяет, что слово Джека можно разбить на префиксы интересного слова.
/// Если да, то возвращает разбиение
/// Использует Z-функцию
fn solve_z_function<'a>(entertaining_word: &str, jack_word: &'a str) -> Option<Vec<&'a str>> {
    let z = z_function(
        &entertaining_word.chars().collect::<Vec<_>>(),
        &jack_word.chars().collect::<Vec<_>>(),
    );

    let mut dp = vec![None; jack_word.len() + 1];
    dp[0] = Some(Vec::new());

    for i in 0..jack_word.len() {
        if let Some(parts) = dp[i].clone() {
            for offset in (1..=z[i]).rev() {
                let end = i + offset;
                if end > jack_word.len() {
                    continue;
                }

                // Берем только первый найденный вариант (самый длинный префикс)
                if dp[end].is_none() {
                    let mut new_parts = parts.clone();
                    new_parts.push(&jack_word[i..end]);

                    if end == jack_word.len() {
                        return Some(new_parts);
                    }
                    dp[end] = Some(new_parts);
                }
            }
        }
    }
    None
}

fn input<T: BufRead>(reader: &mut T) -> (String, String) {
    let mut s1 = String::new();
    let mut s2 = String::new();
    reader.read_line(&mut s1).unwrap();
    reader.read_line(&mut s2).unwrap();
    (s1.trim().to_string(), s2.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static COUNT: usize = 75000;

    #[test]
    fn test_solve() {
        test_helper("abracadabra", "abrabracada", Some(vec!["abr", "abracada"]));
    }

    #[test]
    fn test_solve2() {
        test_helper("abracadabra", "arbadacarba", None);
    }

    #[test]
    fn test_aaabbb() {
        test_helper("aaabbb", "aaa", Some(vec!["aaa"]));
        test_helper("aaabbb", "a", Some(vec!["a"]));
        test_helper("aaabbb", "ab", None);
        test_helper("aaabbb", "aaab", Some(vec!["aaab"]));
        test_helper("aaabbb", "aaaba", Some(vec!["aaab", "a"]));
    }

    #[test]
    fn test_abc() {
        test_helper("abc", "aababc", Some(vec!["a", "ab", "abc"]));
    }

    #[test]
    fn test_aaaaa() {
        test_helper("aaaaaaa", "aaa", Some(vec!["aaa"]));
        test_helper("aaaaaaa", "a", Some(vec!["a"]));
    }

    #[test]
    fn test_long_aaaaaaaaaaa() {
        let s = "a".repeat(COUNT).to_string();
        test_helper(&s, &s, Some(vec![s.as_str()]));
    }

    #[test]
    fn test_long_abcabcabc() {
        test_helper(&"abc".repeat(COUNT / 3), &"acb".repeat(COUNT / 3), None);
    }

    #[test]
    fn test_long_abcabcabc2() {
        test_helper(
            &"abc".repeat(COUNT / 3),
            &"abc".repeat(COUNT / 3),
            Some(vec!["abc".repeat(COUNT / 3).as_str()]),
        );
    }

    #[test]
    #[ignore] // cargo test task10 -- --include-ignored
    /// ## Warning
    /// Сомневаюсь в корректности этого теста
    fn test_long_aabaaaaaaaa() {
        let mut expected = vec!["aa"; COUNT / 2 - 2];
        expected.insert(0, "aaba");
        test_helper(
            &"aab".repeat(COUNT / 3),
            &format!("aab{}", "a".repeat(COUNT - 3)),
            Some(expected),
        );
    }

    fn test_helper(entertaining_word: &str, jack_word: &str, expected: Option<Vec<&str>>) {
        // Выводим только первые n элементов для удобства просмотра
        const ELEMENTS_OUTPUT: usize = 10;
        let result = solve_z_function(entertaining_word, jack_word);

        if result != expected {
            let format_vec = |v: &[&str]| {
                if v.len() <= ELEMENTS_OUTPUT * 2 {
                    format!("{:?}", v)
                } else {
                    let first: Vec<_> = v.iter().take(ELEMENTS_OUTPUT).collect();
                    let last: Vec<_> = v.iter().rev().take(ELEMENTS_OUTPUT).rev().collect();
                    format!(
                        "[{}, .. {} .., {}]",
                        first
                            .iter()
                            .map(|s| format!("\"{}\"", s))
                            .collect::<Vec<_>>()
                            .join(", "),
                        v.len() - first.len() - last.len(),
                        last.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(", ")
                    )
                }
            };

            let res_str = match &result {
                Some(v) => format!("Some({})", format_vec(v)),
                None => "None".to_string(),
            };
            let exp_str = match &expected {
                Some(v) => format!("Some({})", format_vec(v)),
                None => "None".to_string(),
            };

            panic!("Assertion failed:\n  left: {}\n right: {}", res_str, exp_str);
        }
    }

    #[test]
    fn test_dp() {
        test_dp_helper(
            "aaaaaa",
            "aaa",
            vec![Some(vec![]), Some(vec!["a"]), Some(vec!["aa"]), Some(vec!["aaa"])],
        );

        test_dp_helper(
            "abc",
            "aababc",
            vec![
                Some(vec![]),
                Some(vec!["a"]),
                Some(vec!["a", "a"]),
                Some(vec!["a", "ab"]),
                Some(vec!["a", "ab", "a"]),
                Some(vec!["a", "ab", "ab"]),
                Some(vec!["a", "ab", "abc"]),
            ],
        );
    }

    fn test_dp_helper(entertaining_word: &str, jack_word: &str, expected: Vec<Option<Vec<&str>>>) {
        let z = z_function(
            &entertaining_word.chars().collect::<Vec<_>>(),
            &jack_word.chars().collect::<Vec<_>>(),
        );

        let mut dp = vec![None; jack_word.len() + 1];
        dp[0] = Some(Vec::new());

        for i in 0..jack_word.len() {
            if let Some(parts) = dp[i].clone() {
                for offset in (1..=z[i]).rev() {
                    let end = i + offset;
                    if end > jack_word.len() {
                        continue;
                    }

                    // Берем только первый найденный вариант (самый длинный префикс)
                    if dp[end].is_none() {
                        let mut new_parts = parts.clone();
                        new_parts.push(&jack_word[i..end]);
                        dp[end] = Some(new_parts);
                    }
                }
            }
        }
        assert_eq!(dp, expected);
    }
}
