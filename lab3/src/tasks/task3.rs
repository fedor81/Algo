use crate::modules::heap_sort::heap_sort;

pub fn run() {
    let mut competitors = input();
    heap_sort(&mut competitors, true);

    for competitor in competitors {
        println!("{}", competitor.login)
    }
}

fn input() -> Vec<Competitor> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin
        .read_line(&mut buf)
        .expect("Невозможно прочитать строку");

    let count_competitors: u32 = buf
        .trim()
        .parse()
        .expect(&format!("Не удалось преобразовать в u32: {}", buf));

    let mut competitors = Vec::with_capacity(count_competitors as usize);

    for _i in 0..count_competitors {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        competitors.push(Competitor::parse(&buf));
    }

    competitors
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Competitor {
    resolved_tasks: u32,
    fine: u32,
    login: String,
}

impl Competitor {
    pub fn new(login: String, resolved_tasks: u32, fine: u32) -> Self {
        Self {
            resolved_tasks,
            fine,
            login,
        }
    }

    pub fn parse(buf: &str) -> Self {
        let vector: Vec<&str> = buf.trim().split_whitespace().collect();
        let login = vector[0].parse().unwrap();
        let resolved_tasks = vector[1].parse().unwrap();
        let fine = vector[2].parse().unwrap();
        Self {
            login,
            resolved_tasks,
            fine,
        }
    }
}

impl PartialOrd for Competitor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.resolved_tasks.partial_cmp(&other.resolved_tasks) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match other.fine.partial_cmp(&self.fine) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.login.partial_cmp(&self.login)
    }
}

impl Ord for Competitor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_test() {
        let a = Competitor::new("a".to_string(), 1, 1);
        let b = Competitor::new("b".to_string(), 1, 2);
        let c = Competitor::new("alla".to_string(), 2, 1);
        let d = Competitor::new("gena".to_string(), 2, 1);

        assert!(b < a);
        assert!(a <= a.clone());
        assert!(b < c);
        assert!(a < d);
        assert!(d < c);
    }

    #[test]
    fn competitors_sort_test_1() {
        let excepted = vec!["gena", "timofey", "alla", "gosha", "rita"];
        let competitors = vec![
            Competitor::new("alla".to_string(), 4, 100),
            Competitor::new("gena".to_string(), 6, 1000),
            Competitor::new("gosha".to_string(), 2, 90),
            Competitor::new("rita".to_string(), 2, 90),
            Competitor::new("timofey".to_string(), 4, 80),
        ];

        competitors_sort_test_helper(competitors, excepted);
    }

    #[test]
    fn competitors_sort_test_2() {
        let excepted = vec!["alla", "gena", "gosha", "rita", "timofey"];
        let competitors = vec![
            Competitor::new("alla".to_string(), 0, 0),
            Competitor::new("gena".to_string(), 0, 0),
            Competitor::new("gosha".to_string(), 0, 0),
            Competitor::new("rita".to_string(), 0, 0),
            Competitor::new("timofey".to_string(), 0, 0),
        ];

        competitors_sort_test_helper(competitors, excepted);
    }

    #[test]
    fn competitors_sort_test_3() {
        let excepted = vec!["rita", "gena", "gosha", "timofey", "alla"];
        let competitors = vec![
            Competitor::new("alla".to_string(), 0, 999),
            Competitor::new("gena".to_string(), 1, 1),
            Competitor::new("gosha".to_string(), 1, 1),
            Competitor::new("rita".to_string(), 999, 0),
            Competitor::new("timofey".to_string(), 1, 999),
        ];

        competitors_sort_test_helper(competitors, excepted);
    }

    #[test]
    fn competitors_sort_big_test() {
        let mut excepted = Vec::new();
        let mut competitors = Vec::new();
        let count = 10000;

        for i in 0..count {
            excepted.push(format!("competitor_{}", count - i - 1));
            competitors.push(Competitor::new(format!("competitor_{}", i), i, i));
        }

        heap_sort(&mut competitors, true);
        assert_eq!(
            competitors
                .iter()
                .map(|competitor| competitor.login.clone())
                .collect::<Vec<String>>(),
            excepted
        );
    }

    fn competitors_sort_test_helper(mut competitors: Vec<Competitor>, excepted: Vec<&str>) {
        heap_sort(&mut competitors, true);

        assert_eq!(
            competitors
                .iter()
                .map(|competitor| competitor.login.clone())
                .collect::<Vec<String>>(),
            excepted
        );
    }
}
