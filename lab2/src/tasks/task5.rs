use crate::modules::quick_sort::quick_sort_non_recursive;

pub fn run() {
    let mut points = input();
    let result = solve(&mut points);
    println!("{}", result);
}

fn input() -> Vec<Point> {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let count = buf.trim().parse::<usize>().unwrap();
    let mut points = Vec::new();

    for i in 0..count {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        points.push(Point { x, y })
    }

    points
}

#[derive(Debug, Eq, Ord, PartialEq, Hash, PartialOrd, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub fn solve(points: &mut Vec<Point>) -> f64 {
    let polygon = convex_hull(points);
    (calculate_perimeter(&polygon) * 100.0).round() / 100.0
}

/// Вычисляет векторное произведение для определения ориентации тройки точек.
///
/// 1) Если результат > 0: Точки образуют поворот против часовой стрелки (точка B находится "слева" от вектора OA).
/// 2) Если результат < 0: Точки образуют поворот по часовой стрелке (точка B находится "справа" от вектора OA)
/// 3) Если результат = 0: Точки лежат на одной прямой.
fn cross(o: Point, a: Point, b: Point) -> i32 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn convex_hull(points: &mut Vec<Point>) -> Vec<Point> {
    if points.len() <= 3 {
        return points.clone();
    }

    quick_sort_non_recursive(points, false);

    let mut lower = get_half_envelope(&points);
    let mut upper = get_half_envelope(&points.iter().rev().cloned().collect());

    upper.pop();
    lower.pop();

    lower.extend(upper);
    lower
}

/// Собирает половину огибающей оболочки по полярному углу
/// Вектор points должен быть отсортирован!
fn get_half_envelope(points: &Vec<Point>) -> Vec<Point> {
    let mut result = Vec::new();

    for p in points.iter() {
        while result.len() >= 2
            && cross(result[result.len() - 2], result[result.len() - 1], *p) <= 0
        {
            result.pop();
        }
        result.push(*p);
    }

    result
}

fn calculate_perimeter(polygon: &Vec<Point>) -> f64 {
    let mut perimeter = 0.0;

    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];
        let x_diff = p1.x - p2.x;
        let y_diff = p1.y - p2.y;
        perimeter += ((x_diff * x_diff + y_diff * y_diff) as f64).sqrt();
    }

    perimeter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelopes_tests() {
        envelopes_test_helper(
            vec![(2, 2), (5, 1), (8, 2), (5, 3), (3, 4), (5, 8)],
            vec![(2, 2), (5, 1), (8, 2), (5, 8)],
        );
        envelopes_test_helper(
            vec![(-1, -1), (0, 0), (-1, 1), (1, -1), (1, 1)],
            vec![(-1, -1), (-1, 1), (1, -1), (1, 1)],
        );
        envelopes_test_helper(
            vec![(2, 1), (2, 3), (2, 2), (3, 2), (1, 2)],
            vec![(2, 1), (2, 3), (3, 2), (1, 2)],
        );
        envelopes_test_helper(
            vec![(2, 1), (2, 3), (2, 2), (3, 2), (1, 2)],
            vec![(2, 1), (2, 3), (3, 2), (1, 2)],
        );
        envelopes_test_helper(
            vec![
                (1, 5),
                (2, 3),
                (3, 6),
                (1, 1),
                (4, 4),
                (4, 2),
                (6, 4),
                (5, 2),
                (5, 6),
                (3, 1),
            ],
            vec![(1, 1), (1, 5), (3, 1), (3, 6), (5, 2), (5, 6), (6, 4)],
        );
    }

    fn envelopes_test_helper(points: Vec<(i32, i32)>, excepted: Vec<(i32, i32)>) {
        let mut points: Vec<_> = points.into_iter().map(|(x, y)| Point::new(x, y)).collect();
        let mut excepted: Vec<_> = excepted
            .into_iter()
            .map(|(x, y)| Point::new(x, y))
            .collect();

        let mut polygon = convex_hull(&mut points);

        polygon.sort();
        excepted.sort();
        assert_eq!(polygon, excepted)
    }

    #[test]
    fn task5_tests() {
        test_helper(vec![(2, 1), (2, 2), (2, 3), (3, 2), (1, 2)], 5.66);
        test_helper(vec![(0, 1), (0, 0), (-1, -1), (1, -1)], 6.47);
        test_helper(
            vec![(-10, -10), (0, 0), (-10, 10), (10, -10), (10, 10)],
            80.0,
        );
        test_helper(
            vec![
                (2, 2),
                (2, 1),
                (1, 2),
                (1, 1),
                (0, 1),
                (1, 0),
                (1, 3),
                (3, 1),
                (2, 3),
                (3, 2),
                (0, 2),
                (2, 0),
            ],
            9.66,
        );
    }

    #[test]
    fn task5_big_test() {
        let mut points = Vec::new();
        let size = 100;

        for i in 0..=size {
            for j in 0..=size {
                points.push((i, j));
            }
        }

        test_helper(points, (size * 4).into());
    }

    fn test_helper(points: Vec<(i32, i32)>, excepted: f64) {
        let mut points = points.into_iter().map(|(x, y)| Point::new(x, y)).collect();
        let result = solve(&mut points);
        assert_eq!(result, excepted);
    }
}
