pub mod dijkstra;
pub mod bfs;
pub mod dfs;
pub mod floyd_warshall;
pub mod bellman_ford;

pub struct Graph {
    pub adjacencies: Vec<Vec<Edge>>,
}

#[derive(Clone)]
pub struct Edge {
    pub node: usize,
    pub weight: usize,
}

impl Graph {
    pub fn new(number_of_nodes: usize) -> Self {
        Graph {
            adjacencies: vec![Vec::new(); number_of_nodes],
        }
    }

    pub fn add_edge(&mut self, start: usize, end: usize, weight: usize) {
        let e = Edge { node: end, weight };
        self.adjacencies[start].push(e);
    }
}