pub fn run() {
    let (radius, points) = input();
    let stations = solve(radius, &points);

    for station in stations {
        println!("{} {}", station.client_index, station.clients_count);
    }
}

pub fn solve(radius: f64, points: &[Point]) -> Vec<Station> {
    let mut stations: Vec<_> = (0..points.len()).map(|i| Station::new(i)).collect();

    for i in 0..(points.len() - 1) {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance(&points[j]);

            if distance <= radius {
                stations[i].increase_clients_count();
                stations[j].increase_clients_count();
            }
        }
    }

    stations.sort_unstable();
    stations.into_iter().take(10).collect()
}

fn input() -> (f64, Vec<Point>) {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    let mut points = vec![];

    stdin.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let (count, radius) = (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
    );

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        points.push(Point::from_str(&buf));
    }

    (radius, points)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub struct Station {
    client_index: usize,
    clients_count: usize,
}

impl Station {
    pub fn new(client_index: usize) -> Self {
        Station {
            client_index,
            clients_count: 0,
        }
    }

    fn increase_clients_count(&mut self) {
        self.clients_count += 1;
    }
}

impl PartialOrd for Station {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.clients_count.partial_cmp(&self.clients_count) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.client_index.partial_cmp(&other.client_index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn from_str(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        Self {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        assert_eq!(p1.distance(&p2), 5.0);

        let p3 = Point { x: -1.0, y: -1.0 };
        let p4 = Point { x: 1.0, y: 1.0 };
        assert!((p3.distance(&p4) - 2.8284).abs() < 0.0001);
    }

    #[test]
    fn test_point_from_str() {
        let point = Point::from_str("1.5 2.5");
        assert_eq!(point.x, 1.5);
        assert_eq!(point.y, 2.5);
    }

    #[test]
    fn test_task5() {
        test_helper(
            5.0,
            vec![
                (0.0, 0.0),
                (2.0, -2.0),
                (5.0, 10.0),
                (0.0, 100.0),
                (-10.0, 0.0),
            ],
            vec![(0, 1), (1, 1), (2, 0), (3, 0), (4, 0)],
        );
    }

    #[test]
    fn test_from_input() {
        let mut points = vec![];
        let radius = 3.0;
        let input = "3.168070 1.752490
0.500730 6.436580
0.089300 0.112720
2.275440 7.508780
0.779230 4.377090
0.644400 1.381650
1.844920 1.430420
8.079870 5.225030
7.823270 5.317290
1.788400 5.426120";

        for line in input.lines() {
            points.push(Point::from_str(line));
        }

        let expected: Vec<_> = vec![
            (5, 4),
            (1, 3),
            (4, 3),
            (6, 3),
            (9, 3),
            (0, 2),
            (2, 2),
            (3, 2),
            (7, 1),
            (8, 1),
        ]
        .into_iter()
        .map(|(client_index, clients_count)| Station {
            client_index,
            clients_count,
        })
        .collect();

        let stations = solve(radius, &points);

        assert_eq!(stations.len(), expected.len());

        for i in 0..stations.len() {
            assert_eq!(stations[i], expected[i], "radius: {}, index: {}", radius, i);
        }
    }

    #[test]
    fn test_more_than_ten_points() {
        let points: Vec<Point> = (0..15)
            .map(|i| Point {
                x: i as f64,
                y: i as f64,
            })
            .collect();
        let stations = solve(2.0, &points);
        assert_eq!(stations.len(), 10); // Should only return top 10
    }

    #[test]
    fn test_single_point() {
        let points = vec![Point { x: 1.0, y: 1.0 }];
        let stations = solve(5.0, &points);
        assert_eq!(stations.len(), 1);
        assert_eq!(stations[0].clients_count, 0);
        assert_eq!(stations[0].client_index, 0);
    }

    #[test]
    fn test_all_points_in_range() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 2.0, y: 2.0 },
        ];
        let stations = solve(5.0, &points);
        assert_eq!(stations[0].clients_count, 2);
        assert_eq!(stations[1].clients_count, 2);
        assert_eq!(stations[2].clients_count, 2);
    }

    #[test]
    fn test_no_points_in_range() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 20.0, y: 20.0 },
        ];
        let stations = solve(5.0, &points);
        assert_eq!(stations.len(), points.len());
        assert!(stations.iter().all(|s| s.clients_count == 0));
    }

    fn test_helper(radius: f64, clients: Vec<(f64, f64)>, expected: Vec<(usize, usize)>) {
        let clients: Vec<_> = clients.into_iter().map(|(x, y)| Point { x, y }).collect();

        let expected: Vec<_> = expected
            .into_iter()
            .map(|(client_index, clients_count)| Station {
                client_index,
                clients_count,
            })
            .collect();

        let stations = solve(radius, &clients);

        assert_eq!(stations.len(), expected.len());

        for i in 0..stations.len() {
            assert_eq!(stations[i], expected[i], "radius: {}, index: {}", radius, i);
        }
    }
}
