use lab2::modules::quick_sort::quick_sort_cmp;
use std::{collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

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
            heap.change_power(&movement.city, |city_wealth| {
                *city_wealth + billionaire.wealth as u32
            });
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

    let mut result: Vec<_> = leader_days
        .into_iter()
        .filter(|item| item.1 > 0u16)
        .collect();

    quick_sort_cmp(&mut result, |a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => a.1 < b.1,
        other => other.is_lt(),
    });
    result
}

pub struct HashMaxHeap<T, P> {
    data: Vec<Rc<T>>,
    hash_map: HashMap<Rc<T>, DictNode<P>>,
}

struct DictNode<P> {
    power: P,
    index: usize,
}

impl<T, P> HashMaxHeap<T, P>
where
    T: Hash + Eq + Display,
    P: Ord,
{
    pub fn new() -> Self {
        Self {
            data: vec![],
            hash_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: T, power: P) {
        let rc_value = Rc::new(key);

        let index = if let Some(node) = self.hash_map.get_mut(&rc_value) {
            node.power = power;
            node.index
        } else {
            let index = self.data.len();
            self.data.push(rc_value.clone());
            self.hash_map.insert(rc_value, DictNode { power, index });
            index
        };

        self.balance(index);
    }

    pub fn change_power<F>(&mut self, key: &T, set_new_power: F)
    where
        F: FnOnce(&P) -> P,
    {
        let node = self
            .hash_map
            .get_mut(key)
            .expect(&format!("Heap does not contain: {}", key));
        let index = node.index;
        node.power = set_new_power(&node.power);
        self.balance(index);
    }

    pub fn contains(&self, key: &T) -> bool {
        self.hash_map.contains_key(key)
    }

    fn balance_regarding_parent(&mut self, mut index: usize) -> usize {
        while let Some(parent) = self.parent(index) {
            let parent = self.hash_map.get(&self.data[parent]).unwrap();
            let current = self.hash_map.get(&self.data[index]).unwrap();

            if parent.power < current.power {
                index = parent.index;
                self.swap(current.index, parent.index);
            } else {
                break;
            }
        }
        index
    }

    fn swap(&mut self, index: usize, other: usize) {
        let node = self.hash_map.get_mut(&self.data[index]).unwrap();
        node.index = other;

        let other_node = self.hash_map.get_mut(&self.data[other]).unwrap();
        other_node.index = index;

        self.data.swap(index, other);
    }

    pub fn balance(&mut self, mut index: usize) -> usize {
        index = self.balance_regarding_children(index);
        index = self.balance_regarding_parent(index);
        index
    }

    fn balance_regarding_children(&mut self, mut index: usize) -> usize {
        while let Some(left) = self.left(index) {
            let current = self.hash_map.get(&self.data[index]).unwrap();
            let left = self.hash_map.get(&self.data[left]).unwrap();
            let mut max = left;

            if let Some(right) = self.right(index) {
                let right = self.hash_map.get(&self.data[right]).unwrap();

                if left.power < right.power {
                    max = right;
                }
            }

            if current.power < max.power {
                index = max.index;
                self.swap(index, max.index);
            } else {
                break;
            }
        }
        index
    }

    pub fn get_at(&self, index: usize) -> Option<&T> {
        self.data.get(index).map(|v| &**v)
    }

    pub fn get_index(&self, value: &T) -> Option<usize> {
        self.hash_map.get(value).map(|node| node.index)
    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index > 0 {
            Some((index - 1) / 2)
        } else {
            None
        }
    }

    fn left(&self, index: usize) -> Option<usize> {
        let result = 2 * index + 1;
        self.check(result)
    }

    fn check(&self, index: usize) -> Option<usize> {
        if index < self.data.len() {
            Some(index)
        } else {
            None
        }
    }

    fn right(&self, index: usize) -> Option<usize> {
        let result = 2 * index + 2;
        self.check(result)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn remove(&mut self, index: usize) -> T {
        todo!("Операция пока не реализована")
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0).map(|v| &**v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap1() {
        let mut heap = HashMaxHeap::new();
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.left(0), None);

        heap.insert(2, 2);
        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.get_index(&2), Some(0));

        heap.insert(8, 8);
        assert_eq!(heap.peek(), Some(&8));
        assert_eq!(heap.get_index(&2), Some(1));
        assert_eq!(heap.get_index(&8), Some(0));

        heap.insert(4, 4);
        assert_eq!(heap.peek(), Some(&8));
        assert_eq!(heap.get_at(2), Some(&4));

        heap.insert(1, 1);
        assert_eq!(heap.peek(), Some(&8));

        heap.insert(5, 5);
        heap.insert(3, 3);
        assert_eq!(heap.peek(), Some(&8));

        heap.insert(7, 7);
        heap.insert(10, 10);
        assert_eq!(heap.peek(), Some(&10));

        heap.insert(6, 6);
        heap.insert(9, 9);

        assert_eq!(heap.left(0), Some(1));
        assert_eq!(heap.right(0), Some(2));
        assert_eq!(heap.right(1), Some(4));
        assert_eq!(heap.right(2), Some(6));

        assert_eq!(heap.peek(), Some(&10));

        heap.change_power(&10, |_power| 0);
        assert_eq!(heap.peek(), Some(&9));

        heap.change_power(&1, |_power| 100);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.len(), 10);

        heap.insert(1, 1);
        assert_eq!(heap.peek(), Some(&9));
        assert_eq!(heap.len(), 10);
    }

    #[test]
    fn test_heap2() {
        let mut heap = HashMaxHeap::new();

        heap.insert("abc", 2);
        heap.insert("def", 0);
        heap.insert("ghi", 3);
        heap.insert("jkl", 3);
        heap.insert("mno", 4);
        heap.insert("pqr", 1);

        assert_eq!(*heap.peek().unwrap(), "mno");
        heap.change_power(&"def", |_| 100);
        assert_eq!(*heap.peek().unwrap(), "def");
    }

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
        test_helper(
            10,
            vec![("fedor", 10, "moscow")],
            vec![],
            vec![("moscow", 10)],
        );
        test_helper(
            100,
            vec![("fedor", 10, "moscow"), ("oleg", 50, "moscow")],
            vec![(30, "oleg", "sochi")],
            vec![("sochi", 70), ("moscow", 30)],
        );
        test_helper(
            50,
            vec![
                ("boris", 30, "moscow"),
                ("ivan", 20, "spb"),
                ("alex", 25, "sochi"),
            ],
            vec![
                (10, "boris", "spb"),
                (20, "ivan", "moscow"),
                (30, "alex", "moscow"),
            ],
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
