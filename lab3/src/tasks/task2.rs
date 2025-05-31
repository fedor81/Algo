use crate::modules::hash_max_heap::HashMaxHeap;
use lab2::modules::quick_sort::quick_sort_cmp;
use std::{collections::HashMap, rc::Rc};

pub fn run() {
    let config = input();
    let result = solve(config);

    for (city, days) in result {
        println!("{} {}", city, days);
    }
}

#[derive(Debug)]
struct Movement {
    day: u16,
    name: String,
    city: Rc<String>,
}

#[derive(Debug)]
struct Billionaire {
    city: Rc<String>,
    wealth: u8,
}

struct Config {
    count_days: u16,
    billionaires: HashMap<String, Billionaire>,
    cities: HashMap<Rc<String>, u32>,
    movements: Vec<Movement>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            billionaires: HashMap::new(),
            cities: HashMap::new(),
            movements: Vec::new(),
            count_days: 0,
        }
    }

    pub fn add_billionaire(&mut self, name: String, wealth: u8, city: String) {
        let city = Rc::new(city);
        self.cities
            .entry(city.clone())
            .and_modify(|city_wealth| *city_wealth += wealth as u32)
            .or_insert(wealth as u32);
        self.billionaires.insert(name, Billionaire { city, wealth });
    }

    pub fn parse_and_push_billionaire(&mut self, line: &str) {
        let split: Vec<_> = line.split_whitespace().collect();
        let (name, city, wealth) = (
            split[0].to_string(),
            split[1].to_string(),
            split[2].parse().expect("Cannot parse wealth :("),
        );
        self.add_billionaire(name, wealth, city);
    }

    fn add_movement(&mut self, day: u16, name: String, city: String) {
        self.movements.push(Movement {
            day,
            name,
            city: Rc::new(city),
        });
    }

    pub fn parse_and_push_movement(&mut self, line: &str) {
        let split: Vec<_> = line.split_whitespace().collect();
        let (day, name, city) = (
            split[0].parse().expect("Cannot parse day :|"),
            split[1].to_string(),
            split[2].to_string(),
        );
        self.add_movement(day, name, city);
    }
}

fn input() -> Config {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut config = Config::new();

    stdin.read_line(&mut buf).expect("Cannot read line!");
    let count = buf.trim().parse().expect("Cannot parse number!:");

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        config.parse_and_push_billionaire(&buf);
    }

    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let split: Vec<_> = buf.split_whitespace().collect();
    config.count_days = split[0].parse().unwrap();
    let count_registered_movements = split[1].parse().unwrap();

    for i in 0..count_registered_movements {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        config.parse_and_push_movement(&buf);
    }

    config
}

fn solve(mut config: Config) -> Vec<(Rc<String>, u16)> {
    let mut previous_day = 0;
    let mut heap = HashMaxHeap::new();
    let mut leader_days = HashMap::new();

    for (city, wealth) in config.cities {
        heap.insert(city, wealth);
    }

    for movement in &config.movements {
        if movement.day != previous_day {
            let difference_days = movement.day - previous_day;
            previous_day = movement.day;

            let max_city = heap.peek().unwrap().clone();
            leader_days
                .entry(max_city)
                .and_modify(|count| *count += difference_days)
                .or_insert(difference_days);
        }

        let billionaire = config.billionaires.get_mut(&movement.name).unwrap();

        if !heap.contains(&movement.city) {
            heap.insert(movement.city.clone(), billionaire.wealth as u32);
        } else {
            heap.change_power(&movement.city, |city_wealth| *city_wealth + billionaire.wealth as u32);
        }

        heap.change_power(&billionaire.city, |city_wealth| {
            *city_wealth - billionaire.wealth as u32
        });

        billionaire.city = movement.city.clone();
    }

    if previous_day != config.count_days {
        let difference_days = config.count_days - previous_day;
        let max_city = heap.peek().unwrap().clone();
        leader_days
            .entry(max_city)
            .and_modify(|count| *count += difference_days)
            .or_insert(difference_days);
    }

    let mut result: Vec<_> = leader_days.into_iter().filter(|item| item.1 > 0u16).collect();

    quick_sort_cmp(&mut result, |a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => a.1 < b.1,
        other => other.is_lt(),
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut config = Config::new();
        config.count_days = 10;
        config.parse_and_push_billionaire("fedor moscow 100");
        config.parse_and_push_movement("1 fedor peterburg");

        let result = solve(config);
        assert_eq!(*result[0].0, "moscow");
        assert_eq!(result[0].1, 1);

        assert_eq!(*result[1].0, "peterburg");
        assert_eq!(result[1].1, 9);
    }

    #[test]
    fn tests_task2() {
        test_helper(10, vec![("fedor", 10, "moscow")], vec![], vec![("moscow", 10)]);
        test_helper(
            100,
            vec![("fedor", 10, "moscow"), ("oleg", 50, "moscow")],
            vec![(30, "oleg", "sochi")],
            vec![("sochi", 70), ("moscow", 30)],
        );
        test_helper(
            50,
            vec![("boris", 30, "moscow"), ("ivan", 20, "spb"), ("alex", 25, "sochi")],
            vec![(10, "boris", "spb"), (20, "ivan", "moscow"), (30, "alex", "moscow")],
            vec![("moscow", 30), ("spb", 20)],
        );
        test_helper(
            20,
            vec![("anna", 40, "moscow"), ("peter", 30, "spb")],
            vec![(5, "anna", "spb"), (5, "peter", "moscow")],
            vec![("moscow", 5), ("spb", 15)],
        );
        test_helper(
            15,
            vec![("rich", 100, "moscow")],
            vec![(1, "rich", "kiev")],
            vec![("moscow", 1), ("kiev", 14)],
        );
        test_helper(
            10,
            vec![("wealthy", 50, "paris")],
            vec![(10, "wealthy", "london")],
            vec![("paris", 10)],
        );
    }

    #[test]
    fn big_test() {
        let count = 50000;
        let mut config = Config::new();

        config.count_days = count;
        config.parse_and_push_billionaire("fedor moscow 100");

        for i in 1..10000 {
            config.parse_and_push_billionaire(&format!("name_{} city_{} 1", i, i));
        }

        for i in 1..count {
            if i % 4 == 0 {
                config.parse_and_push_movement(format!("{} fedor moscow", i).as_str());
            } else if i % 4 == 1 {
                config.parse_and_push_movement(format!("{} fedor peterburg", i).as_str());
            } else if i % 4 == 2 {
                config.parse_and_push_movement(format!("{} fedor sochi", i).as_str());
            } else if i % 4 == 3 {
                config.parse_and_push_movement(format!("{} fedor paris", i).as_str());
            }
        }

        let mut expected = vec![
            ("peterburg", count / 4),
            ("moscow", count / 4),
            ("sochi", count / 4),
            ("paris", count / 4),
        ];

        let result = solve(config);
        expected.sort();

        for i in 0..result.len() {
            assert_eq!(*result[i].0, expected[i].0);
            debug_assert_eq!(result[i].1, expected[i].1, "{}", result[i].0);
        }
    }

    fn test_helper(
        count_days: u16,
        billionaires: Vec<(&str, u8, &str)>,
        movements: Vec<(u16, &str, &str)>,
        mut expected: Vec<(&str, u16)>,
    ) {
        let mut config = Config::new();
        config.count_days = count_days;

        for (name, wealth, city) in billionaires {
            config.add_billionaire(name.to_string(), wealth, city.to_string());
        }

        for (day, name, city) in movements {
            config.add_movement(day, name.to_string(), city.to_string());
        }

        let result = solve(config);
        assert_eq!(result.len(), expected.len());
        expected.sort();

        for i in 0..result.len() {
            assert_eq!(*result[i].0, expected[i].0);
            assert_eq!(result[i].1, expected[i].1);
        }
    }
}
