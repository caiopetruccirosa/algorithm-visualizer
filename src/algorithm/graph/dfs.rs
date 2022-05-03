use crate::algorithm::graph::Graph;

pub fn path_to(graph: &Graph, start: usize, goal: usize) -> Option<Vec<usize>> {
    let mut previous: Vec<usize> = (0..graph.adjacencies.len()).map(|_| start).collect();
    let mut visited: Vec<bool> = (0..graph.adjacencies.len()).map(|_| false).collect();
    let mut stack = Vec::<usize>::new();

    visited[start] = true;
    stack.push(start);
    
    while let Some(node) = stack.pop() {
        if node == goal { 
            let mut backwards = Vec::<usize>::new();

            let mut aux = node;
            while aux != start {
                backwards.push(aux);
                aux = previous[aux];
            }
            backwards.push(start);

            return Some(backwards.into_iter().rev().collect());
        }

        for edge in &graph.adjacencies[node] {
            if !visited[edge.node] {
                stack.push(edge.node);
                visited[edge.node] = true;
                previous[edge.node] = node;
            }
        }
    }

    None
}