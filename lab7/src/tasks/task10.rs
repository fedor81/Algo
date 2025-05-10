use std::io::{BufRead, BufReader};

use crate::modules::disjoint_set_union::DisjointSetUnion;

pub fn run() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let (mut graph, power_stations) = input(&mut reader);
    let result = solve(&mut graph, &power_stations);
    println!("{}", result);
}

fn solve(graph: &mut NondirectionalGraph, power_stations: &Vec<usize>) -> usize {
    let mut dsu = DisjointSetUnion::new();
    let mut connected = vec![false; graph.count_nodes()];
    let mut cost = 0;

    graph.edges_mut().sort_by(|a, b| a.weight.cmp(&b.weight));

    for &power_station in power_stations {
        dsu.add(power_station);
        connected[power_station] = true;
    }

    for edge in graph.edges() {
        if connected[edge.from] && connected[edge.to] {
            continue;
        }

        let root_a = dsu.find_or_add(edge.from);
        let root_b = dsu.find_or_add(edge.to);
        dsu.unite(root_a, root_b);

        connected[edge.from] = true;
        connected[edge.to] = true;
        cost += edge.weight;
    }

    cost
}

fn input<T: BufRead>(input: &mut T) -> (NondirectionalGraph, Vec<usize>) {
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();

    let mut split = buf.split_whitespace();
    let (cities, power_stations) = (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse::<usize>().unwrap(),
    );

    buf.clear();
    input.read_line(&mut buf).unwrap();

    let power_stations: Vec<_> = buf
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect();
    let mut graph = NondirectionalGraph::new(cities);

    for i in 0..cities {
        buf.clear();
        input.read_line(&mut buf).unwrap();

        let split = buf.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        for j in (i + 1)..cities {
            graph.add_edge(i, j, split[j]);
        }
    }

    (graph, power_stations)
}

struct NondirectionalGraph {
    nodes: usize,
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

impl NondirectionalGraph {
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes,
            edges: Vec::new(),
        }
    }

    pub fn count_nodes(&self) -> usize {
        self.nodes
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn edges_mut(&mut self) -> &mut [Edge] {
        &mut self.edges
    }

    pub fn add_edge(&mut self, node1: usize, node2: usize, weight: usize) {
        self.edges.push(Edge::new(node1, node2, weight));
    }
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self { from, to, weight }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_with_input() {
        let s = "4 2
1 4
0 2 4 3
2 0 5 2
4 5 0 1
3 2 1 0
";
        let (mut graph, power_stations) = input(&mut Cursor::new(s));
        assert_eq!(&power_stations, &vec![0, 3]);
        assert_eq!(graph.edges().len(), 6);
        assert_eq!(graph.count_nodes(), 4);
        assert_eq!(graph.edges()[0], Edge::new(0, 1, 2));
        assert_eq!(graph.edges()[1], Edge::new(0, 2, 4));
        assert_eq!(graph.edges()[2], Edge::new(0, 3, 3));
        assert_eq!(graph.edges()[3], Edge::new(1, 2, 5));
        assert_eq!(graph.edges()[4], Edge::new(1, 3, 2));
        assert_eq!(graph.edges()[5], Edge::new(2, 3, 1));
        assert_eq!(solve(&mut graph, &power_stations), 3);
    }

    #[test]
    fn test_all_stations() {
        let s = "3 3
1 2 3
0 10 20
10 0 30
20 30 0";
        let (mut graph, power_stations) = input(&mut Cursor::new(s));
        assert_eq!(solve(&mut graph, &power_stations), 0);
    }

    #[test]
    fn test_one_without_station() {
        let s = "3 2
1 2
0 5 10
5 0 3
10 3 0";
        let (mut graph, power_stations) = input(&mut Cursor::new(s));
        assert_eq!(solve(&mut graph, &power_stations), 3);
    }

    #[test]
    fn test_direct_connection() {
        let s = "4 1
1
0 1 1 1
1 0 100 100
1 100 0 100
1 100 100 0";
        let (mut graph, power_stations) = input(&mut Cursor::new(s));
        assert_eq!(solve(&mut graph, &power_stations), 3);
    }

    #[test]
    fn test_hard() {
        let s = "5 2
1 3
0 1 4 5 6
1 0 2 7 8
4 2 0 3 9
5 7 3 0 10
6 8 9 10 0";
        let (mut graph, power_stations) = input(&mut Cursor::new(s));
        assert_eq!(solve(&mut graph, &power_stations), 10);
    }

    #[test]
    fn test_non_direct_connection() {
        let s = "4 2
1 3
0 100 100 1
100 0 1 100
100 1 0 100
1 100 100 0";
        let (mut graph, power_stations) = input(&mut Cursor::new(s));
        assert_eq!(solve(&mut graph, &power_stations), 2);
    }
}
