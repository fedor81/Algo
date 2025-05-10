use std::io::{BufRead, BufReader};

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let (graph, start) = input(&mut reader);

    if let Some(ways) = solve(&graph, start) {
        for (i, way) in ways.into_iter().enumerate() {
            match way {
                Reach::Cost(cost) => println!("{}", cost),
                Reach::Unreachable => println!("UNREACHABLE"),
            }
        }
    } else {
        println!("IMPOSSIBLE");
    }

    todo!()
}

pub struct Graph {
    nodes: usize,
    edges: Vec<Edge>,
}

pub struct Edge {
    from: usize,
    to: usize,
    weight: i32,
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes,
            edges: Vec::with_capacity(nodes),
        }
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn nodes_count(&self) -> usize {
        self.nodes
    }

    /// Функция не проверяет ребра на дубликаты
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.edges.push(Edge { from, to, weight });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reach {
    Cost(i32),
    Unreachable,
}

/// Использует алгоритм Беллмана-Форда для нахождения кратчайших путей до каждой вершины или None
fn solve(graph: &Graph, start: usize) -> Option<Vec<Reach>> {
    let mut dist = vec![i32::MAX; graph.nodes_count()];
    dist[start] = 0;

    // После N-1 итерации массив должен содержать кратчайшие пути
    for i in 0..graph.nodes_count() - 1 {
        for edge in graph.edges() {
            if dist[edge.from] == i32::MAX {
                continue;
            }

            if dist[edge.from] + edge.weight < dist[edge.to] {
                dist[edge.to] = dist[edge.from] + edge.weight
            }
        }
    }

    // Проверяем наличие отрицательных циклов
    for edge in graph.edges() {
        if dist[edge.from] == i32::MAX {
            continue;
        }

        // Если хотя бы одно расстояние улучшится, значит, в графе есть отрицательный цикл
        if dist[edge.from] + edge.weight < dist[edge.to] {
            return None;
        }
    }

    Some(
        dist.into_iter()
            .map(|x| {
                if x == i32::MAX {
                    Reach::Unreachable
                } else {
                    Reach::Cost(x)
                }
            })
            .collect(),
    )
}

fn input<T: BufRead>(input: &mut T) -> (Graph, usize) {
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();

    let vec = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    let (nodes, edges, start) = (vec[0], vec[1], vec[2]);
    let mut graph = Graph::new(nodes);

    for i in 0..edges {
        buf.clear();
        input.read_line(&mut buf).unwrap();
        let vec = buf.split_whitespace().collect::<Vec<_>>();
        let (from, to, weight) = (
            vec[0].parse().unwrap(),
            vec[1].parse::<usize>().unwrap(),
            vec[2].parse::<i32>().unwrap(),
        );
        graph.add_edge(from, to, weight);
    }

    (graph, start)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn speed_test() {
        let start = 0;
        let mut graph = Graph::new(800);
        let mut expected = vec![Reach::Unreachable; 800];

        for from in 0..600 {
            for offset in 1..=50 {
                graph.add_edge(from, from + offset, 0);
                expected[from + offset] = Reach::Cost(0);
            }
        }

        let ways = solve(&graph, start);
    }

    #[test]
    fn test_0_100_200_unr() {
        let s = "4 5 0
0 1 100
1 2 100
2 0 -100
0 2 1000
3 1 15
";
        let (graph, start) = input(&mut Cursor::new(s));
        let ways = solve(&graph, start);
        assert_eq!(
            ways,
            Some(vec![
                Reach::Cost(0),
                Reach::Cost(100),
                Reach::Cost(200),
                Reach::Unreachable,
            ])
        );
    }

    #[test]
    fn test_impossible() {
        let s = "3 3 0
0 1 5
1 2 8
2 0 -20
";
        let (graph, start) = input(&mut Cursor::new(s));
        let ways = solve(&graph, start);
        assert_eq!(ways, None);
    }

    #[test]
    fn test_zero_weight_cycle() {
        let s = "3 3 0
0 1 0
1 2 0
2 0 0
";
        let (graph, start) = input(&mut Cursor::new(s));
        let ways = solve(&graph, start);
        assert_eq!(ways, Some(vec![Reach::Cost(0), Reach::Cost(0), Reach::Cost(0)]));
    }

    #[test]
    fn test_multiple_paths_same_destination() {
        let s = "3 4 0
0 1 5
0 2 10
1 2 3
2 1 2
";
        let (graph, start) = input(&mut Cursor::new(s));
        let ways = solve(&graph, start);
        assert_eq!(ways, Some(vec![Reach::Cost(0), Reach::Cost(5), Reach::Cost(8)]));
    }

    #[test]
    fn test_all_reachable() {
        let s = "4 5 0
0 1 1
0 2 4
1 2 2
1 3 6
2 3 3
";
        let (graph, start) = input(&mut Cursor::new(s));
        let ways = solve(&graph, start);
        assert_eq!(
            ways,
            Some(vec![Reach::Cost(0), Reach::Cost(1), Reach::Cost(3), Reach::Cost(6)])
        );
    }

    #[test]
    fn test_disconnected_graph() {
        let s = "4 2 0
0 1 5
2 3 3
";
        let (graph, start) = input(&mut Cursor::new(s));
        let ways = solve(&graph, start);
        assert_eq!(
            ways,
            Some(vec![
                Reach::Cost(0),
                Reach::Cost(5),
                Reach::Unreachable,
                Reach::Unreachable
            ])
        );
    }
}
