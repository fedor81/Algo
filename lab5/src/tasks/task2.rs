use std::{
    fs,
    io::ErrorKind,
    time::{SystemTime, UNIX_EPOCH},
};

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

#[derive(Debug)]
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
    // Генерируем уникальное имя файла для тестов
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("./task2_{}.db", timestamp);
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
    #[ignore]
    fn test_task2() {
        test_helper(
            &vec![
                "ADD JW SJXO",
                "ADD RZBR YMW",
                "ADD ADX LVT",
                "ADD LKFLG UWM",
                "PRINT ADX",
                "UPDATE HNTP JQPVG",
                "ADD QURWB MEGW",
                "PRINT QURWB",
                "DELETE MB",
                "DELETE ADX",
            ],
            &vec!["ADX LVT", "ERROR", "QURWB MEGW", "ERROR"],
        );

        test_helper(
            &vec![
                "ADD RWJSN JFTF",
                "ADD ZDH GOON",
                "ADD FCDS TCAY",
                "ADD FCDS TCAY",
                "ADD HMGVI BWK",
                "ADD JTDU TLWWN",
                "ADD IXRJ ERF",
                "ADD IAOD GRDO",
                "PRINT IXRJ",
                "PRINT JTDU",
                "PRINT IXRJ",
                "UPDATE ZDH IOX",
                "PRINT ZDH",
                "ADD GVWU RTA",
                "DELETE ZDH",
                "ADD FCDS IVFJV",
            ],
            &vec![
                "ERROR",
                "IXRJ ERF",
                "JTDU TLWWN",
                "IXRJ ERF",
                "ZDH IOX",
                "ERROR",
            ],
        );
    }

    #[test]
    #[ignore]
    fn test_basic_operations() {
        test_helper(
            &vec![
                "ADD key1 value1",
                "PRINT key1",
                "UPDATE key1 value2",
                "PRINT key1",
                "DELETE key1",
                "PRINT key1",
            ],
            &vec!["key1 value1", "key1 value2", "ERROR"],
        );
    }

    #[test]
    #[ignore]
    fn test_duplicate_adds() {
        test_helper(
            &vec![
                "ADD key1 value1",
                "ADD key1 value2", // Should fail - duplicate key
            ],
            &vec!["ERROR"],
        );
    }

    #[test]
    #[ignore]
    fn test_invalid_updates_and_deletes() {
        test_helper(
            &vec![
                "UPDATE nonexistent value1", // Should fail - key doesn't exist
                "DELETE nonexistent",        // Should fail - key doesn't exist
                "PRINT nonexistent",         // Should fail - key doesn't exist
            ],
            &vec!["ERROR", "ERROR", "ERROR"],
        );
    }

    #[test]
    #[ignore]
    fn test_multiple_operations() {
        test_helper(
            &vec![
                "ADD key1 value1",
                "ADD key2 value2",
                "PRINT key1",
                "PRINT key2",
                "UPDATE key1 newvalue1",
                "DELETE key2",
                "PRINT key1",
                "PRINT key2",
            ],
            &vec!["key1 value1", "key2 value2", "key1 newvalue1", "ERROR"],
        );
    }

    #[test]
    #[ignore]
    fn test_operations_order() {
        test_helper(
            &vec![
                "ADD key1 value1",
                "DELETE key1",
                "ADD key1 value2", // Should work after deletion
                "PRINT key1",
            ],
            &vec!["key1 value2"],
        );
    }

    fn test_helper(operations: &[&str], expected: &[&str]) {
        let operations: Vec<_> = operations
            .iter()
            .map(|s| Operations::from_str(&s.to_uppercase()))
            .collect();
        let output = solve(operations);
        assert_eq!(output.len(), expected.len());

        for (out, exp) in output.iter().zip(expected.iter()) {
            assert_eq!(out, &exp.to_uppercase());
        }
    }
}
