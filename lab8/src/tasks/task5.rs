pub fn run() {
    let mut text = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut text).unwrap();
    println!("{}", create_palindrome(&text));
}

pub fn create_palindrome(text: &str) -> String {
    let text_vec = text.chars().collect::<Vec<_>>();
    let mirrored_part: String;

    // Если строка состоит из одного символа - продублируем
    if text.len() == 1 {
        return text.to_string() + text;
    }

    // Ищем самый длинный палиндромный суффикс
    if let Some(split) = find_rightmost_palindrome(&text_vec) {
        // Если палиндромнй суффикс не начинается с первого символа - делим, разворачиваем
        if split != 0 {
            mirrored_part = text_vec[..split].iter().rev().collect();
        }
        // Если вся строка — палиндром
        else {
            // Если вся строка состоит из одинаковых символов - просто допишем один в конец
            if text_vec.iter().all(|&c| c == text_vec[0]) {
                mirrored_part = String::from(text_vec[0]);
            }
            // Если строка состоит из разных символов - делим, разворачиваем
            else {
                mirrored_part = text_vec[1..].iter().collect();
            }
        }
    }
    // Если в строке нет палиндромных суффиксов
    else {
        mirrored_part = text_vec[..text_vec.len() - 1].iter().rev().collect();
    }

    text.to_string() + &mirrored_part
}

/// Находит максимально длинный палиндромный суффикс в строке, если он есть, и возвращает индекс его начала
fn find_rightmost_palindrome(text: &[char]) -> Option<usize> {
    for start in 0..text.len() {
        let mut left = start;
        let mut right = text.len() - 1;
        let mut is_palindrome = true;

        if left == right {
            continue;
        }

        while left < right {
            if text[left] != text[right] {
                is_palindrome = false;
                break;
            }
            left += 1;
            right -= 1;
        }

        if is_palindrome {
            return Some(start);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(create_palindrome("No"), "NoN");
        assert_eq!(create_palindrome("OnLine"), "OnLineniLnO");
        assert_eq!(create_palindrome("AbabaAab"), "AbabaAababA");
        assert_eq!(create_palindrome("Nooo"), "NoooN");
        assert_eq!(create_palindrome("NooN"), "NooNooN");
        assert_eq!(create_palindrome("NoN"), "NoNoN");
        assert_eq!(create_palindrome("Noo"), "NooN");
        assert_eq!(create_palindrome("N"), "NN");
        assert_eq!(create_palindrome("NN"), "NNN");
        assert_eq!(create_palindrome("NNN"), "NNNN");
        assert_eq!(create_palindrome("NNNN"), "NNNNN");
        assert_eq!(create_palindrome("NNNNN"), "NNNNNN");
        assert_eq!(create_palindrome("telelel"), "telelelet");
        assert_eq!(create_palindrome("holl"), "holloh");
        assert_eq!(create_palindrome("abba"), "abbabba");
        assert_eq!(create_palindrome("abccba"), "abccbabccba");
        assert_eq!(create_palindrome("aba"), "ababa");
        assert_eq!(create_palindrome("abcacba"), "abcacbabcacba");
    }

    #[test]
    fn test_find_even_rightmost_palindrome() {
        assert_eq!(find_rightmost_palindrome(&"Nono".chars().collect::<Vec<_>>()), Some(1));
        assert_eq!(find_rightmost_palindrome(&"Noo".chars().collect::<Vec<_>>()), Some(1));
        assert_eq!(find_rightmost_palindrome(&"NoN".chars().collect::<Vec<_>>()), Some(0));
        assert_eq!(find_rightmost_palindrome(&"NooN".chars().collect::<Vec<_>>()), Some(0));
        assert_eq!(find_rightmost_palindrome(&"No".chars().collect::<Vec<_>>()), None);
        assert_eq!(
            find_rightmost_palindrome(&"aaaaaaa".chars().collect::<Vec<_>>()),
            Some(0)
        );
        assert_eq!(find_rightmost_palindrome(&"aa".chars().collect::<Vec<_>>()), Some(0));
        assert_eq!(
            find_rightmost_palindrome(&"Lineni".chars().collect::<Vec<_>>()),
            Some(1)
        );
        assert_eq!(
            find_rightmost_palindrome(&"AbabaAab".chars().collect::<Vec<_>>()),
            Some(3)
        );
    }
}
