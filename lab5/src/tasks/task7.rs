use crate::modules::dictionary::Dict;

pub fn run() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut dict = Dict::with_capacity(n);

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let command: Vec<&str> = buf.trim().split_whitespace().collect();

        match command[0] {
            "get" => {
                let key: i32 = command[1].parse().unwrap();
                match dict.get(key) {
                    Some(value) => println!("{}", value),
                    None => println!("None"),
                }
            }
            "put" => {
                let key: i32 = command[1].parse().unwrap();
                let value: i32 = command[2].parse().unwrap();
                dict.insert(key, value);
            }
            "delete" => {
                let key: i32 = command[1].parse().unwrap();
                match dict.remove(key) {
                    Some(value) => println!("{}", value),
                    None => println!("None"),
                }
            }
            _ => println!("Unknown command"),
        }
    }
}
