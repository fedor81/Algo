use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use crate::modules::merge_sort::merge_sort;

pub fn run() {
    let filename = "./src/tasks/task5.rs";
    let output = "./input/output5.txt";
    solve(filename, output, 4);
}

fn solve(input: &str, output: &str, memory_size: usize) {
    let buffers_count = split_into_buffers(input, memory_size);
    let mut buffers = VecDeque::from_iter(0..buffers_count);

    while buffers.len() > 1 {
        let result = get_buffer_name(buffers.len());
        buffers.push_front(buffers.len());

        let buf1 = get_buffer_name(buffers.pop_back().unwrap());
        let buf2 = get_buffer_name(buffers.pop_back().unwrap());

        merge_buffers(&buf1, &buf2, &result);
    }

    let result_number = buffers.pop_front().unwrap();
    fs::rename(get_buffer_name(result_number), output).expect("Не вышло записать результат");
    delete_buffers(buffers_count);
}

fn delete_buffers(buffers_count: usize) {
    for i in 0..=buffers_count {
        fs::remove_file(get_buffer_name(i)).unwrap_or_else(|_| {});
    }
}

fn get_buffer_name(number: usize) -> String {
    format!("./buf{}.txt", number)
}

/// Возвращает количество созданных буферов
fn split_into_buffers(filename: &str, lines_in_buffer: usize) -> usize {
    let file = File::open(filename).expect(&format!("Невозможно прочитать файл: {}", filename));
    let reader = BufReader::new(file);

    let mut vector = Vec::with_capacity(lines_in_buffer);
    let mut buffer_number = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                vector.push(line);

                if vector.len() == lines_in_buffer {
                    sort_and_write(&get_buffer_name(buffer_number), &vector);

                    buffer_number += 1;
                    vector.clear();
                }
            }
            Err(e) => {
                eprintln!("Ошибка при чтении строки: {}", e);
            }
        }
    }

    if vector.len() != 0 {
        sort_and_write(&get_buffer_name(buffer_number), &vector);
        buffer_number += 1;
    }

    buffer_number
}

fn sort_and_write(filename: &str, lines: &Vec<String>) {
    let sort_vec = merge_sort(&lines.iter().collect(), false);
    let mut file = File::create(filename).expect("Не удалось создать буфер");

    for line in sort_vec {
        file.write(line.as_bytes())
            .expect("Не удалось записать в буфер");
        file.write("\n".as_bytes()).unwrap();
    }
}

fn merge_buffers(buf1: &str, buf2: &str, result: &str) {
    let buf1 = File::open(buf1).expect(&format!("Не удалось открыть буфер: {}", buf1));
    let buf2 = File::open(buf2).expect(&format!("Не удалось открыть буфер: {}", buf2));
    let mut result = File::create(result).expect(&format!(
        "Не удалось открыть результирующий буфер: {}",
        result
    ));

    let mut reader1 = BufReader::new(buf1).lines();
    let mut reader2 = BufReader::new(buf2).lines();

    let mut line1 = reader1.next();
    let mut line2 = reader2.next();

    loop {
        match (&line1, &line2) {
            (Some(Ok(val1)), Some(Ok(val2))) => {
                if val1 < val2 {
                    result.write(val1.as_bytes()).unwrap();
                    line1 = reader1.next();
                } else {
                    result.write(val2.as_bytes()).unwrap();
                    line2 = reader2.next();
                }
            }
            (Some(Ok(val)), None) => {
                result.write(val.as_bytes()).unwrap();
                line1 = reader1.next();
            }
            (None, Some(Ok(val))) => {
                result.write(val.as_bytes()).unwrap();
                line2 = reader2.next();
            }
            _ => break,
        }

        result.write(b"\n").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task7() {
        test_helper("./input/input5.txt", "./input/output5.txt");
        test_helper("./src/tasks/task7.rs", "./input/output5.txt");
        test_helper("./src/tasks/task5.rs", "./input/output5.txt");
    }

    fn test_helper(input: &str, output: &str) {
        solve(input, output, 10);

        let result = read_file(&output);
        let mut expected = read_file(&input);
        expected.sort();

        assert_eq!(result, expected)
    }

    fn read_file(filename: &str) -> Vec<String> {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(|line| line.unwrap()).collect()
    }
}
