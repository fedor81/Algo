use crate::modules::heap::MinHeap;

pub fn run() {
    let numbers = input();
    let result = solve(&numbers);
    println!("{result}");
}

pub fn solve(numbers: &[f64]) -> f64 {
    let mut heap = MinHeap::new();
    let mut commission = 0.;

    for n in numbers {
        heap.push(*n);
    }

    while heap.len() > 1 {
        let n1 = heap.pop().unwrap();
        let n2 = heap.pop().unwrap();
        let sum = n1 + n2;
        commission += sum * 0.01;
        heap.push(sum);
    }

    round(commission)
}

/// Округление до двух знаков после запятой
fn round(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}

fn input() -> Vec<f64> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let count = buf.trim().parse().unwrap();
    let mut numbers = Vec::new();

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let number = buf.trim().parse().unwrap();
        numbers.push(number);
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task2() {
        test_helper(&vec![10, 30], 0.4);
        test_helper(&vec![10, 30, 50], 1.3);
        test_helper(&vec![1, 2, 3, 4, 5], 0.33);
        test_helper(&vec![2, 10, 100, 30, 7, 4, 15, 2, 15, 80], 6.52);
    }

    fn test_helper(numbers: &[u32], expected: f64) {
        assert_eq!(
            solve(&numbers.iter().map(|&n| n as f64).collect::<Vec<f64>>()),
            expected
        );
    }
}
