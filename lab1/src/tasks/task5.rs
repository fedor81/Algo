use crate::modules::big_int::BigInt;

pub fn run() {
    let (number1, operation, number2) = input();
    let result = match operation {
        '+' => number1 + number2,
        '-' => number1 - number2,
        _ => panic!("Invalid operation"),
    };
    println!("{}", result);
}

fn input() -> (BigInt, char, BigInt) {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let number1 = BigInt::from_str(&buf);

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let operation = buf.chars().next().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let number2 = BigInt::from_str(&buf);

    (number1, operation, number2)
}
