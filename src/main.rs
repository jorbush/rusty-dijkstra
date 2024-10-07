use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Edge {
    target: usize,
    weight: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    adjacency_list: Vec<Vec<Edge>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if let Some(&d) = distances.get(&position) {
            if cost > d {
                continue;
            }
        }

        for edge in graph.get_neighbors(position) {
            let next = State {
                cost: cost + edge.weight,
                position: edge.target,
            };
            if next.cost < *distances.get(&next.position).unwrap_or(&usize::MAX) {
                distances.insert(next.position, next.cost);
                heap.push(next);
            }
        }
    }

    distances
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adjacency_list: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: usize) {
        self.adjacency_list[u].push(Edge {
            target: v,
            weight: w,
        });
    }

    fn get_neighbors(&self, u: usize) -> &Vec<Edge> {
        &self.adjacency_list[u]
    }

    fn show(&self) {
        for (u, edges) in self.adjacency_list.iter().enumerate() {
            print!("{}: ", u);
            for edge in edges {
                print!("({}, {}) ", edge.target, edge.weight);
            }
            println!();
        }
    }
}

fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, 1);
    graph.add_edge(0, 2, 2);
    graph.add_edge(1, 3, 3);
    graph.add_edge(2, 4, 4);
    graph.add_edge(3, 4, 5);
    graph.add_edge(4, 0, 6);

    println!("{:?}", graph.get_neighbors(0));
    graph.show();

    let distances = dijkstra(&graph, 0);
    for (node, distance) in distances {
        println!("Node: {}, Distance: {}", node, distance);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 2);
        graph.add_edge(1, 3, 3);
        graph.add_edge(2, 4, 4);
        graph.add_edge(3, 4, 5);
        graph.add_edge(4, 0, 6);

        let distances = dijkstra(&graph, 0);
        assert_eq!(distances[&0], 0);
        assert_eq!(distances[&1], 1);
        assert_eq!(distances[&2], 2);
        assert_eq!(distances[&3], 4);
        assert_eq!(distances[&4], 6);
    }
}
