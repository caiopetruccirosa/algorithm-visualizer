use crate::algorithm::graph::Graph;

pub fn all_pairs_distances(graph: &Graph) -> Vec<Vec<usize>> {
    let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; graph.adjacencies.len()]; graph.adjacencies.len()];

    for v in 0..distances.len() {
        distances[v][v] = 0;
    }

    for v in 0..graph.adjacencies.len() {
        for edge in &graph.adjacencies[v] {
            distances[v][edge.node] = edge.weight;
        }
    }

    for k in 0..distances.len() {
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                if distances[i][j] > distances[i][k] + distances[j][k] {
                    distances[i][j] = distances[i][k] + distances[j][k];
                }
            }
        }
    }

    distances
}