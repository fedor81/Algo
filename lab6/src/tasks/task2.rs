use std::collections::VecDeque;
use std::fmt::Display;

use lab3::modules::heap_sort;

pub fn run() {
    let config = input();
    let (profit, passengers) = solve(config);

    println!("{}", profit);

    for p in passengers {
        println!("{}", p);
    }
}

#[derive(Debug)]
pub struct Config {
    count_stops: usize,
    count_places_in_bus: usize,
    fare: u32,
    passengers: Vec<Passenger>,
}

impl Config {
    pub fn count_passengers(&self) -> usize {
        self.passengers.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Passenger {
    from: u16,
    to: u16,
    index: u16,
}

impl Display for Passenger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({} {})", self.from, self.to))
    }
}

impl Passenger {
    pub fn from_str(s: &str, index: u16) -> Self {
        let mut iter = s.split_whitespace();
        Self {
            from: iter.next().unwrap().parse().unwrap(),
            to: iter.next().unwrap().parse().unwrap(),
            index,
        }
    }
}

fn input() -> Config {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let count_stops = iter.next().unwrap().parse().unwrap();
    let count_places_in_bus = iter.next().unwrap().parse().unwrap();
    let count_passengers = iter.next().unwrap().parse().unwrap();
    let fare = iter.next().unwrap().parse().unwrap();

    let mut passengers = vec![];

    for i in 1..=count_passengers {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        passengers.push(Passenger::from_str(&buf, i));
    }

    Config {
        count_stops,
        count_places_in_bus,
        fare,
        passengers,
    }
}

pub fn solve(mut config: Config) -> (u32, Vec<u16>) {
    let mut profit = 0u32;
    let mut passengers = vec![];
    let mut bus_deque: VecDeque<Passenger> = VecDeque::with_capacity(config.count_places_in_bus);

    heap_sort::heap_sort_cmp(&mut config.passengers, |p1, p2| {
        p1.to < p2.to || (p1.to == p2.to && p1.from < p2.from)
    });

    for passenger_at_stop in config.passengers {
        if let Some(passenger_in_bus) = bus_deque.front().cloned() {
            if passenger_in_bus.to <= passenger_at_stop.from {
                bus_deque.pop_front();
            }
        }

        if bus_deque.len() < config.count_places_in_bus {
            passengers.push(passenger_at_stop.index);
            bus_deque.push_back(passenger_at_stop);
            profit += config.fare;
        }
    }

    (profit, passengers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let mut passengers =
            convert_to_passenger_vec(&vec![(1, 4), (2, 6), (1, 5), (2, 3), (4, 6), (3, 6)]);
        let mut expected =
            convert_to_passenger_vec(&vec![(2, 3), (1, 4), (1, 5), (2, 6), (3, 6), (4, 6)]);

        heap_sort::heap_sort_cmp(&mut passengers, |p1, p2| {
            p1.to < p2.to || (p1.to == p2.to && p1.from < p2.from)
        });

        for (actual, expect) in passengers.into_iter().zip(expected) {
            assert_eq!((actual.from, actual.to), (expect.from, expect.to));
        }
    }

    fn convert_to_passenger_vec(passengers: &[(u16, u16)]) -> Vec<Passenger> {
        passengers
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, (from, to))| Passenger {
                from,
                to,
                index: (i + 1) as u16,
            })
            .collect()
    }

    #[test]
    fn sequence_test() {
        let mut passengers: Vec<_> = (0..100).into_iter().map(|i| (i, i + 1)).collect();

        test_helper(
            150,
            1,
            1,
            &mut passengers,
            100,
            &mut (1..=100).into_iter().collect::<Vec<u16>>(),
        );
        test_helper(
            150,
            10,
            1,
            &mut passengers,
            100,
            &mut (1..=100).into_iter().collect::<Vec<u16>>(),
        );
    }

    #[test]
    fn test_full_bus_capacity() {
        let passengers = convert_to_passenger_vec(&vec![
            (1, 3),
            (1, 2),
            (1, 4), // Three passengers at same start
        ]);

        let config = Config {
            count_stops: 5,
            count_places_in_bus: 2, // Only 2 seats
            fare: 10,
            passengers,
        };

        let (profit, selected_passengers) = solve(config);
        assert_eq!(profit, 20); // Only 2 passengers can board
        assert_eq!(selected_passengers.len(), 2);
    }

    #[test]
    fn test_overlapping_routes() {
        let passengers = convert_to_passenger_vec(&vec![
            (1, 5), // Long route
            (2, 3), // Short route
            (3, 4), // Another short route
        ]);

        let config = Config {
            count_stops: 6,
            count_places_in_bus: 2,
            fare: 5,
            passengers,
        };

        let (profit, selected_passengers) = solve(config);
        assert_eq!(profit, 15); // All can be accommodated due to timing
    }

    #[test]
    fn test_task2() {
        test_helper(
            6,
            2,
            9,
            &vec![(1, 4), (2, 6), (1, 5), (2, 3), (4, 6), (3, 6)],
            36,
            &mut vec![1, 5, 6, 4],
        );
        test_helper(
            20,
            3,
            1,
            &vec![
                (1, 9),
                (0, 3),
                (0, 20), /* Не попадает */
                (1, 4),
                (10, 14),
                (9, 11),
                (4, 7),
                (12, 15),
                (5, 7),
                (1, 20), /* Не попадает */
                (12, 13),
            ],
            9,
            &mut vec![1, 2, 4, 5, 6, 7, 8, 9, 11],
        );
    }

    fn test_helper(
        count_stops: usize,
        count_places_in_bus: usize,
        fare: u32,
        passengers: &[(u16, u16)],
        expected_profit: u32,
        expected_passengers: &mut [u16],
    ) {
        let config = Config {
            count_stops,
            count_places_in_bus,
            fare,
            passengers: convert_to_passenger_vec(passengers),
        };

        let (actual_profit, mut actual_passengers) = solve(config);

        assert_eq!(actual_profit, expected_profit);
        assert_eq!(actual_passengers.len(), expected_passengers.len());

        actual_passengers.sort();
        expected_passengers.sort();

        for (actual, &mut expected) in actual_passengers.into_iter().zip(expected_passengers) {
            assert_eq!(actual, expected);
        }
    }
}
