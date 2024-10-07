#[derive(Debug, Clone, Copy)]
struct Edge {
    target: usize,
    weight: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    adjacency_list: Vec<Vec<Edge>>,
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
}
