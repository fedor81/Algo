use std::io::BufRead;

pub fn run() {
    let input = input();
    let result = solve(input);
    println!("result: {}", result);
}

fn input() -> Vec<Item> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let _count = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let expression = lines.next().unwrap().unwrap();
    parse_line(&expression)
}

fn solve(expression: Vec<Item>) -> i32 {
    let mut stack = Vec::new();

    for item in expression {
        match item {
            Item::Number(n) => stack.push(n as i32),
            Item::Operator(operator) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(operator.apply(a, b));
            }
        }
    }

    stack.pop().unwrap()
}

fn parse_line(mut expression: &str) -> Vec<Item> {
    expression = expression.trim();
    let mut vector = Vec::with_capacity(expression.len() / 2 + 1);

    for c in expression.split_whitespace() {
        let c = c.chars().next().unwrap();
        if c.is_digit(10) {
            vector.push(Item::new_number(c as u8 - '0' as u8));
        } else {
            vector.push(Item::new_operator(Operator::from_char(c)));
        }
    }

    vector
}

#[derive(Debug, PartialEq)]
enum Item {
    Number(u8),
    Operator(Operator),
}

impl Item {
    fn new_number(value: u8) -> Item {
        Item::Number(value)
    }

    fn new_operator(operator: Operator) -> Item {
        Item::Operator(operator)
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
}

impl Operator {
    pub fn from_char(c: char) -> Operator {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            _ => panic!("Invalid operator"),
        }
    }

    pub fn apply(&self, a: i32, b: i32) -> i32 {
        match self {
            Operator::Add => a + b,
            Operator::Sub => a - b,
            Operator::Mul => a * b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            parse_line(&"0 1 2 3 + - *"),
            vec![
                Item::new_number(0),
                Item::new_number(1),
                Item::new_number(2),
                Item::new_number(3),
                Item::new_operator(Operator::Add),
                Item::new_operator(Operator::Sub),
                Item::new_operator(Operator::Mul),
            ]
        );

        assert_eq!(
            parse_line("1 2 +"),
            vec![
                Item::new_number(1),
                Item::new_number(2),
                Item::new_operator(Operator::Add)
            ]
        );

        assert_eq!(
            parse_line("5 3 2 * -"),
            vec![
                Item::new_number(5),
                Item::new_number(3),
                Item::new_number(2),
                Item::new_operator(Operator::Mul),
                Item::new_operator(Operator::Sub)
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Invalid operator")]
    fn test_invalid_operator() {
        parse_line("1 2 $");
    }

    #[test]
    fn tests_task7() {
        // Basic operations
        test_helper("1 2 +", 3); // Addition
        test_helper("5 3 -", 2); // Subtraction
        test_helper("4 2 *", 8); // Multiplication

        // Multiple operations
        test_helper("1 2 3 + +", 6); // Sequential addition
        test_helper("5 3 2 - -", 4); // Sequential subtraction
        test_helper("2 3 4 * *", 24); // Sequential multiplication

        // Mixed operations
        test_helper("3 4 + 2 *", 14); // (3 + 4) * 2
        test_helper("5 2 * 3 +", 13); // (5 * 2) + 3
        test_helper("9 5 - 2 *", 8); // (9 - 5) * 2

        // More complex expressions
        test_helper("1 2 + 3 4 + *", 21); // (1 + 2) * (3 + 4)
        test_helper("5 3 * 2 4 * -", 7); // (5 * 3) - (2 * 4)
        test_helper("2 3 4 + * 5 -", 9); // (2 * (3 + 4)) - 5

        // Edge cases
        test_helper("0 0 +", 0); // Zero addition
        test_helper("0 5 *", 0); // Multiplication by zero
        test_helper("1 0 *", 0); // Multiplication by zero
        test_helper("5 5 -", 0); // Subtraction to zero
    }

    fn test_helper(expression: &str, expected: i32) {
        let expression = parse_line(expression);
        let result = solve(expression);
        assert_eq!(result, expected);
    }

    #[test]
    fn big_test1() {
        let count = 1000000;
        let mut expression = Vec::with_capacity(count);

        for i in 0..(count / 5) {
            expression.push(Item::new_number(2));
            expression.push(Item::new_number(1));
            expression.push(Item::new_number(2));
            expression.push(Item::new_operator(Operator::Mul));
            expression.push(Item::new_operator(Operator::Sub));
        }

        let result = solve(expression);
        assert_eq!(result, 0);
    }

    #[test]
    fn big_test2() {
        let count = 1000000;
        let mut expression = Vec::with_capacity(count);

        for i in 0..(count / 2) {
            expression.push(Item::new_number(1));
        }

        for i in 0..(count / 2 - 1) {
            expression.push(Item::new_operator(Operator::Mul));
        }

        let result = solve(expression);
        assert_eq!(result, 1);
    }
}
