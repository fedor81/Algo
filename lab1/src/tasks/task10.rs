use std::collections::{HashMap, HashSet};

pub fn run() {
    let (number_competitors, first_team_size, familiar_competitors) = input();
    let result = solve(number_competitors, first_team_size, familiar_competitors);

    for player in result {
        print!("{} ", player);
    }
    println!()
}

fn input() -> (u16, u16, HashMap<u16, HashSet<u16>>) {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let numbers: Vec<u16> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    if numbers.len() != 3 {
        panic!("Неверный ввод");
    }

    let (number_competitors, first_team_size, familiar_count) =
        (numbers[0], numbers[1], numbers[2]);

    let mut familiar_competitors = Vec::new();

    for _i in 0..familiar_count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut competitors = buf.split_whitespace().map(|s| s.parse().unwrap());
        familiar_competitors.push((competitors.next().unwrap(), competitors.next().unwrap()))
    }

    (
        number_competitors,
        first_team_size,
        convert_to_familiar_hashmap(familiar_competitors),
    )
}

pub fn convert_to_familiar_hashmap(pairs: Vec<(u16, u16)>) -> HashMap<u16, HashSet<u16>> {
    let mut familiar_competitors = HashMap::new();

    for (comp1, comp2) in pairs {
        familiar_competitors
            .entry(comp1)
            .or_insert_with(HashSet::new)
            .insert(comp2);
        familiar_competitors
            .entry(comp2)
            .or_insert_with(HashSet::new)
            .insert(comp1);
    }

    familiar_competitors
}

pub fn solve(
    number_competitors: u16,
    first_team_size: u16,
    familiar_competitors: HashMap<u16, HashSet<u16>>,
) -> Vec<u16> {
    let mut team1 = Team::new(first_team_size, &familiar_competitors);
    let mut team2 = Team::new(number_competitors - first_team_size, &familiar_competitors);

    (team1, team2) = find_max_familiar_teams(1, number_competitors, team1, team2);

    team1.competitors.into_iter().collect()
}

struct Team<'a> {
    pub competitors: HashSet<u16>,
    power: u16,
    max_size: usize,
    familiar_competitors: &'a HashMap<u16, HashSet<u16>>,
}

impl<'a> Team<'a> {
    pub fn add(&mut self, competitor: u16) {
        self.competitors.insert(competitor);

        if let Some(familiar_set) = self.familiar_competitors.get(&competitor) {
            self.power += familiar_set.intersection(&self.competitors).count() as u16;
        }
    }

    fn new(max_size: u16, familiar_competitors: &'a HashMap<u16, HashSet<u16>>) -> Self {
        Self {
            competitors: HashSet::new(),
            max_size: (max_size as usize),
            familiar_competitors,
            power: 0,
        }
    }
}

impl<'a> Clone for Team<'a> {
    fn clone(&self) -> Self {
        Self {
            competitors: self.competitors.clone(),
            power: self.power,
            max_size: self.max_size,
            familiar_competitors: self.familiar_competitors,
        }
    }
}

fn find_max_familiar_teams<'a, 'b>(
    current: u16,
    max_player_number: u16,
    mut team1: Team<'a>,
    mut team2: Team<'b>,
) -> (Team<'a>, Team<'b>) {
    if current <= max_player_number {
        let can_add_to_team1 = team1.competitors.len() < team1.max_size;
        let can_add_to_team2 = team2.competitors.len() < team2.max_size;

        if can_add_to_team1 && can_add_to_team2 {
            let team1_clone = team1.clone();
            let team2_clone = team2.clone();

            team1.add(current);
            team2.add(current);

            let next = current + 1;

            let result1 = find_max_familiar_teams(next, max_player_number, team1, team2_clone);
            let result2 = find_max_familiar_teams(next, max_player_number, team1_clone, team2);

            if result1.0.power + result1.1.power >= result2.0.power + result2.1.power {
                result1
            } else {
                result2
            }
        } else if can_add_to_team1 {
            team1.add(current);
            find_max_familiar_teams(current + 1, max_player_number, team1, team2)
        } else {
            team2.add(current);
            find_max_familiar_teams(current + 1, max_player_number, team1, team2)
        }
    } else {
        (team1, team2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_test() {
        let mut expected = vec![2, 5, 4];
        let mut result = solve(
            5,
            3,
            convert_to_familiar_hashmap(vec![(1, 3), (2, 5), (5, 4)]),
        );

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_cases() {
        test_helper(2, 1, vec![(1, 2)], vec![1]);
        test_helper(3, 1, vec![(1, 2)], vec![3]);
        test_helper(3, 1, vec![(3, 2)], vec![1]);
        test_helper(3, 2, vec![(3, 2)], vec![2, 3]);
        test_helper(4, 2, vec![(3, 2), (1, 4)], vec![1, 4]);
        test_helper(4, 3, vec![(3, 2), (1, 2)], vec![1, 2, 3]);
        test_helper(4, 1, vec![(3, 2), (1, 2)], vec![4]);
        test_helper(5, 3, vec![(1, 2), (1, 3), (1, 4), (4, 5)], vec![1, 2, 3]);
    }

    fn test_helper(
        number_competitors: u16,
        first_team_size: u16,
        familiar_competitors: Vec<(u16, u16)>,
        mut excepted: Vec<u16>,
    ) {
        let mut result = solve(
            number_competitors,
            first_team_size,
            convert_to_familiar_hashmap(familiar_competitors),
        );

        result.sort();
        excepted.sort();
        assert_eq!(result, excepted);
    }
}
