use std::io::{BufRead, BufReader};

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let (s1, s2) = input(&mut reader);
    let result = solve(&s1, &s2);

    if let Some(result) = result {
        println!("{}", result);
    } else {
        println!("-1");
    }
}

pub fn solve(s: &str, t: &str) -> Option<usize> {
    if s == t {
        return Some(0);
    }

    let s_chars = s.chars().collect::<Vec<_>>();
    let t_chars = t.chars().collect::<Vec<_>>();
    let length = s_chars.len();

    for offset in (0..length).rev() {
        let mut find = true;
        for i in 0..(2 * length) {
            if s_chars[(i + offset) % length] != t_chars[i % length] {
                find = false;
                break;
            }
        }
        if find {
            return Some(length - offset);
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

/// Определяет число циклических сдвигов для получения строки T из S, либо выясняет,
/// что T не может быть получена из S.
///
/// ## Алгоритм Кнута-Морриса-Пратта (KMP)
///
/// 1. **Построить `lps`-массив** для подстроки \( T \).  
/// 2. **Поиск в тексте \( S \)** с использованием `lps`:  
///    - Сравниваем символы \( S \) и \( T \).  
///    - Если символы совпадают — двигаемся дальше.  
///    - Если нет — используем `lps`, чтобы "перепрыгнуть" уже проверенную часть.
///
/// ## Пример работы
///
/// ### Дано:
/// - \( S = "ABABDABACDABABCABAB" \),  
/// - \( T = "ABABCABAB" \),  
/// - `lps = [0, 0, 1, 2, 0, 1, 2, 3, 4]`.  
///
/// ### Пошаговый поиск:
/// 1. Совпадение до `"ABAB"`, затем несоответствие (`'D' != 'C'`).  
/// 2. `j` сбрасывается до `lps[3] = 2` (потому что `lps[j-1] = lps[3] = 2`).  
/// 3. Продолжаем сравнение с `T[2]` (вместо начала).  
/// 4. Находим полное совпадение на позиции `10`.  
pub fn find_shift_kmp(s: &str, t: &str) -> Option<usize> {
    if s == t {
        return Some(0);
    }

    let s_chars = s.chars().collect::<Vec<_>>();
    let t_chars = t.chars().collect::<Vec<_>>();
    let length = s_chars.len();

    if length != t_chars.len() {
        return None;
    }

    let mut i = 0; // Индекс для S + S
    let mut j = 0; // Индекс для T
    let lps = compute_lps(&t_chars);

    while i < 2 * length {
        // Эмулируем S + S через S[i % N]
        if s_chars[i % length] == t_chars[j] {
            i += 1;
            j += 1;
            if j == length {
                return Some((i - j) % length);
            }
        } else {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    None
}

/// Строит префикс-функцию для строки
///
/// Префикс-функция для подстроки T — это массив lps(longest prefix suffix), где:
/// `lps[i]` = длина наибольшего собственного префикса `T[0..i]`, который также является суффиксом.
///
/// ### Пример для \( T = "ABABCABAB" \):
///
/// | Индекс (i) | Подстрока \(T[0..i]\) | Наибольший префикс = суффикс | `lps[i]` |
/// |------------|--------------------------|-----------------------------|----------|
/// | 0          | "A"                      | "" (нет)                    | 0        |
/// | 1          | "AB"                     | ""                          | 0        |
/// | 2          | "ABA"                    | "A"                         | 1        |
/// | 3          | "ABAB"                   | "AB"                        | 2        |
/// | 4          | "ABABC"                  | ""                          | 0        |
/// | 5          | "ABABCA"                 | "A"                         | 1        |
/// | 6          | "ABABCAB"                | "AB"                        | 2        |
/// | 7          | "ABABCABA"               | "ABA"                       | 3        |
/// | 8          | "ABABCABAB"              | "ABAB"                      | 4        |
///
/// **Итоговый `lps = [0, 0, 1, 2, 0, 1, 2, 3, 4]`**
fn compute_lps(pattern: &[char]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut length = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    return lps;
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
        let actual = solve(&s1, &s2);

        assert_eq!(actual, Some(9));
    }

    #[test]
    fn test_aaaaaa() {
        assert_eq!(solve("aaaaaa", "aaaaaa"), Some(0));
    }

    #[test]
    fn test_abaaaa() {
        assert_eq!(solve("abaaaa", "aaaaba"), Some(3));
        assert_eq!(solve("abaaaa", "aaabaa"), Some(2));
        assert_eq!(solve("abaaaa", "baaaaa"), Some(5));
        assert_eq!(solve("abaaaa", "aaaaaa"), None);
    }

    #[test]
    fn test_no_possible_shift() {
        assert_eq!(solve("hello", "world"), None);
        assert_eq!(solve("abcde", "bcdea"), Some(4));
        assert_eq!(solve("abcde", "cdeab"), Some(3));
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
            assert_eq!(solve(s1, s2), expected, "Failed for s1: {}, s2: {}", s1, s2);
        }
    }

    #[test]
    fn test_special_patterns() {
        // Test with repeating patterns
        assert_eq!(solve("aaaa", "aaaa"), Some(0));
        assert_eq!(solve("abab", "abab"), Some(0));
        assert_eq!(solve("abab", "baba"), Some(1));
    }

    #[test]
    fn test_big_strings() {
        assert_eq!(solve(&"b".repeat(COUNT), &"a".repeat(COUNT)), None);
        assert_eq!(solve(&"ab".repeat(COUNT / 2), &"ba".repeat(COUNT / 2)), Some(1));
        assert_eq!(
            solve(
                &format!("a{}", "b".repeat(COUNT - 1)),
                &format!("{}a", "b".repeat(COUNT - 1))
            ),
            Some(1)
        );
    }

    #[test]
    fn test_random_strings() {
        let s = Alphanumeric.sample_string(&mut rand::rng(), COUNT);
        let shift = rand::random_range(1..COUNT);
        let t = format!("{}{}", &s[shift..], &s[..shift]);
        assert_eq!(solve(&s, &t), Some(shift));
    }

    #[test]
    fn test_compute_lps() {
        assert_eq!(
            compute_lps(&['A', 'B', 'A', 'B', 'C', 'A', 'B', 'A', 'B']),
            vec![0, 0, 1, 2, 0, 1, 2, 3, 4]
        );
        assert_eq!(compute_lps(&['A', 'A', 'A', 'A']), vec![0, 1, 2, 3]);
        assert_eq!(compute_lps(&['A', 'B', 'C']), vec![0, 0, 0]);
    }
}
