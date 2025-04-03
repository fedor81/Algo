use std::{fs, io::ErrorKind};

use crate::modules::hash_database::HashDatabase;

pub fn run() {
    let operations = input();
    let output = solve(operations);

    for s in output {
        println!("{}", s);
    }
}

fn input() -> Vec<Operations> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();
    let mut commands = Vec::new();

    for i in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        commands.push(Operations::from_str(&buf));
    }

    commands
}

pub enum Operations {
    Add { key: String, value: String },
    Delete { key: String },
    Update { key: String, value: String },
    Print { key: String },
}

impl Operations {
    fn from_str(input: &str) -> Self {
        let iter: Vec<_> = input.split_whitespace().collect();
        match iter[0] {
            "ADD" => Self::Add {
                key: iter[1].to_string(),
                value: iter[2].to_string(),
            },
            "DELETE" => Self::Delete {
                key: iter[1].to_string(),
            },
            "UPDATE" => Self::Update {
                key: iter[1].to_string(),
                value: iter[2].to_string(),
            },
            "PRINT" => Self::Print {
                key: iter[1].to_string(),
            },
            _ => panic!("Неверная команда"),
        }
    }
}

pub fn solve(operations: Vec<Operations>) -> Vec<String> {
    let mut output = vec![];
    let path = "task2.db";
    let mut db =
        HashDatabase::new(&path, operations.len() as u64).expect("Не удалось создать базу данных");

    for op in operations {
        match op {
            Operations::Add { key, value } => {
                if let Err(e) = db.add(key, value) {
                    match e.kind() {
                        ErrorKind::AlreadyExists => output.push("ERROR".to_string()),
                        _ => panic!("{}", e),
                    }
                };
            }
            Operations::Delete { key } => {
                if let Err(e) = db.remove(&key) {
                    match e.kind() {
                        ErrorKind::NotFound => output.push("ERROR".to_string()),
                        _ => panic!("{}", e),
                    }
                }
            }
            Operations::Update { key, value } => {
                if let Err(e) = db.update(&key, value) {
                    match e.kind() {
                        ErrorKind::NotFound => output.push("ERROR".to_string()),
                        _ => panic!("{}", e),
                    }
                }
            }
            Operations::Print { key } => match db.get(&key) {
                Ok(option) => match option {
                    Some((key, value)) => output.push(format!("{} {}", key, value)),
                    None => output.push("ERROR".to_string()),
                },
                Err(e) => panic!("{}", e),
            },
        };
    }

    fs::remove_file(&path).unwrap();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task2() {}
}
