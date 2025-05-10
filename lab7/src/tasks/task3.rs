use std::collections::{HashMap, VecDeque};

pub fn run() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let (max_values, need_milliliters) = parse_input(&input);
    let result = solve(max_values, need_milliliters);

    if let Some(result) = result {
        println!("{}", result);
    } else {
        println!("OOPS");
    }
}

pub fn solve(max_values: State, need_milliliters: u16) -> Option<usize> {
    let mut hash_map = HashMap::new();
    let mut queue = VecDeque::new();

    let init = State::new(max_values.big_flask, 0, 0);
    queue.push_back(init);
    hash_map.insert(init, 0);

    while let Some(state) = queue.pop_front() {
        if state.big_flask == need_milliliters {
            return Some(hash_map[&state]);
        }

        // Либо заливаем все содержимое колбы, либо заливаем сколько поместится

        // Переливаем big -> first
        let mut pour = state.big_flask.min(max_values.first_flask - state.first_flask);
        if pour > 0 {
            let new_state = State::new(state.big_flask - pour, state.first_flask + pour, state.second_flask);
            if !hash_map.contains_key(&new_state) {
                hash_map.insert(new_state, hash_map[&state] + 1);
                queue.push_back(new_state);
            }
        }

        // Переливаем big -> second
        pour = state.big_flask.min(max_values.second_flask - state.second_flask);
        if pour > 0 {
            let new_state = State::new(state.big_flask - pour, state.first_flask, state.second_flask + pour);
            if !hash_map.contains_key(&new_state) {
                hash_map.insert(new_state, hash_map[&state] + 1);
                queue.push_back(new_state);
            }
        }

        // Переливаем first -> second
        pour = state.first_flask.min(max_values.second_flask - state.second_flask);
        if pour > 0 {
            let new_state = State::new(state.big_flask, state.first_flask - pour, state.second_flask + pour);
            if !hash_map.contains_key(&new_state) {
                hash_map.insert(new_state, hash_map[&state] + 1);
                queue.push_back(new_state);
            }
        }

        // Переливаем first -> big
        pour = state.first_flask.min(max_values.big_flask - state.big_flask);
        if pour > 0 {
            let new_state = State::new(state.big_flask + pour, state.first_flask - pour, state.second_flask);
            if !hash_map.contains_key(&new_state) {
                hash_map.insert(new_state, hash_map[&state] + 1);
                queue.push_back(new_state);
            }
        }

        // Переливаем second -> big
        pour = state.second_flask.min(max_values.big_flask - state.big_flask);
        if pour > 0 {
            let new_state = State::new(state.big_flask + pour, state.first_flask, state.second_flask - pour);
            if !hash_map.contains_key(&new_state) {
                hash_map.insert(new_state, hash_map[&state] + 1);
                queue.push_back(new_state);
            }
        }

        // Переливаем second -> first
        pour = state.second_flask.min(max_values.first_flask - state.first_flask);
        if pour > 0 {
            let new_state = State::new(state.big_flask, state.first_flask + pour, state.second_flask - pour);
            if !hash_map.contains_key(&new_state) {
                hash_map.insert(new_state, hash_map[&state] + 1);
                queue.push_back(new_state);
            }
        }
    }

    None
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct State {
    big_flask: u16,
    first_flask: u16,
    second_flask: u16,
}

impl State {
    pub fn new(big_flask: u16, first_flask: u16, second_flask: u16) -> Self {
        Self {
            big_flask,
            first_flask,
            second_flask,
        }
    }
}

fn parse_input(s: &str) -> (State, u16) {
    let vec: Vec<_> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    (
        State {
            big_flask: vec[0],
            first_flask: vec[1],
            second_flask: vec[2],
        },
        vec[3],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(State::new(6, 3, 2), 6), Some(0));
        assert_eq!(solve(State::new(6, 3, 2), 15), None);
        assert_eq!(solve(State::new(6, 3, 2), 1), Some(2));
        assert_eq!(solve(State::new(6, 3, 2), 2), Some(3));
        assert_eq!(solve(State::new(6, 3, 2), 3), Some(1));
        assert_eq!(solve(State::new(6, 3, 2), 4), Some(1));
        assert_eq!(solve(State::new(6, 3, 2), 5), Some(3));

        // (10, 0, 0) -> (7, 3, 0) -> (7, 0, 3) -> (4, 3, 3)
        assert_eq!(solve(State::new(10, 3, 5), 4), Some(3));
        assert_eq!(solve(State::new(10, 3, 5), 2), Some(2));

        // (10, 0, 0) -> (3, 0, 7) -> (3, 2, 5) -> (5, 0, 5)
        assert_eq!(solve(State::new(10, 2, 7), 5), Some(3));
        assert_eq!(solve(State::new(5, 5, 5), 3), None);

        solve(State::new(2000, 3, 1990), 1999);
        assert_eq!(solve(State::new(500, 5, 5), 3), None);
    }
}
