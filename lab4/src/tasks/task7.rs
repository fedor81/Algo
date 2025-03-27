use lab1::modules::big_int::BigInt;
use num_bigint::BigUint;

pub fn run() {
    let (numbers, queries) = input();
    let results = solve_on_my_bigint(numbers, queries);

    for result in results {
        println!("{}", result);
    }
}

fn input() -> (Vec<u32>, Vec<Query>) {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let (length, queries_count) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );

    let mut arr = Vec::with_capacity(length);
    let mut queries = Vec::with_capacity(queries_count);

    for i in 0..length {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let n = buf.trim().parse().unwrap();
        arr.push(n);
    }

    for i in 0..queries_count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();

        match iter.next().unwrap() {
            "1" => queries.push(Query::Sum {
                start: iter.next().unwrap().parse().unwrap(),
                end: iter.next().unwrap().parse().unwrap(),
            }),
            "2" => queries.push(Query::Replace {
                index: iter.next().unwrap().parse().unwrap(),
                element: iter.next().unwrap().parse().unwrap(),
            }),
            _ => panic!("Invalid query type"),
        }
        {}
    }

    (arr, queries)
}

enum Query {
    Sum { start: usize, end: usize },
    Replace { index: usize, element: u32 },
}

fn solve_on_my_bigint(numbers: Vec<u32>, queries: Vec<Query>) -> Vec<BigInt> {
    vec![]
}

fn solve_on_other_bigint(numbers: Vec<u32>, queries: Vec<Query>) -> Vec<BigUint> {
    vec![]
}
