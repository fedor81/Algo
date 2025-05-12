use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let (text, words) = input(&mut reader);
    let result = can_segment_text(&text, &words);

    if result {
        println!("YES");
    } else {
        println!("NO");
    }
}

pub fn can_segment_text(text: &str, words: &HashSet<String>) -> bool {
    let text_len = text.len();
    let max_word_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let mut can_segment = vec![false; text_len + 1];
    can_segment[0] = true; // Пустая строка

    for i in 1..=text_len {
        let start = if i > max_word_len { i - max_word_len } else { 0 };
        for j in start..i {
            if can_segment[j] && words.contains(&text[j..i].to_string()) {
                can_segment[i] = true;
                break;
            }
        }
    }
    can_segment[text_len]
}

fn input<T: BufRead>(reader: &mut T) -> (String, HashSet<String>) {
    let mut text = String::new();
    reader.read_line(&mut text).unwrap();

    let mut count = String::new();
    reader.read_line(&mut count).unwrap();
    let count: usize = count.trim().parse().unwrap();
    let mut words = HashSet::new();

    for i in 0..count {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        words.insert(line.trim().to_string());
    }

    (text.trim().to_string(), words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_task() {
        let s = "examiwillpasstheexam
5
will
pass
the
exam
i
";
        let (text, words) = input(&mut Cursor::new(s));
        assert_eq!(can_segment_text(&text, &words), true);
    }

    #[test]
    fn test_abacaba_false() {
        test_helper(&"abacaba", &vec!["abac", "caba"], false);
    }

    #[test]
    fn test_abacaba_true() {
        test_helper(&"abacaba", &vec!["abac", "caba", "aba"], true);
    }

    fn test_helper(text: &str, words: &[&str], expected: bool) {
        let words_set: HashSet<String> = words.iter().map(|&w| w.to_string()).collect();
        assert_eq!(can_segment_text(text, &words_set), expected);
    }
}
