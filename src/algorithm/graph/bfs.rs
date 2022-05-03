use crate::algorithm::graph::Graph;
use std::collections::VecDeque;

pub fn distance_to(graph: &Graph, start: usize, goal: usize) -> Option<usize> {
    let mut costs: Vec<usize> = (0..graph.adjacencies.len()).map(|_| 0).collect();
    let mut visited: Vec<bool> = (0..graph.adjacencies.len()).map(|_| false).collect();
    let mut queue = VecDeque::<usize>::new();

    visited[start] = true;
    queue.push_back(start);
    
    while let Some(node) = queue.pop_front() {
        if node == goal {
            return Some(costs[goal]);
        }

        for edge in &graph.adjacencies[node] {
            if !visited[edge.node] {
                queue.push_back(edge.node);
                visited[edge.node] = true;
                costs[edge.node] = costs[node] + 1;
            }
        }
    }

    None
}

pub fn shortest_path_to(graph: &Graph, start: usize, goal: usize) -> Option<Vec<usize>> {
    let mut previous: Vec<usize> = (0..graph.adjacencies.len()).map(|_| start).collect();
    let mut visited: Vec<bool> = (0..graph.adjacencies.len()).map(|_| false).collect();
    let mut queue = VecDeque::<usize>::new();

    visited[start] = true;
    queue.push_back(start);
    
    while let Some(node) = queue.pop_front() {
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
                queue.push_back(edge.node);
                visited[edge.node] = true;
                previous[edge.node] = node;
            }
        }
    }

    None
}