use crate::algorithm::graph::Graph;
use std::collections::binary_heap::BinaryHeap;
use std::cmp::{Ordering, Reverse};

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    node: usize,
    cost: usize,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn distance_to(graph: &Graph, start: usize, goal: usize) -> Option<usize> {
    let mut costs: Vec<usize> = (0..graph.adjacencies.len()).map(|_| usize::MAX).collect();    
    let mut queue = BinaryHeap::<Reverse<NodeCost>>::new();

    queue.push(Reverse(NodeCost { node: start, cost: 0 }));
    costs[start] = 0;

    while let Some(Reverse(NodeCost { node, cost })) = queue.pop() {
        if node == goal { return Some(cost); }

        if cost > costs[node] { continue; }

        for edge in &graph.adjacencies[node] {
            let new_cost = cost + edge.weight;

            if new_cost < costs[edge.node] {
                queue.push(Reverse(NodeCost { cost: new_cost, node: edge.node }));
                costs[edge.node] = new_cost;
            }
        }
    }

    None
}

pub fn shortest_path_to(graph: &Graph, start: usize, goal: usize) -> Option<Vec<usize>> {
    let mut costs: Vec<usize> = (0..graph.adjacencies.len()).map(|_| usize::MAX).collect();
    let mut previous: Vec<usize> = (0..graph.adjacencies.len()).map(|_| start).collect();  
    let mut queue = BinaryHeap::<Reverse<NodeCost>>::new();

    queue.push(Reverse(NodeCost { node: start, cost: 0 }));
    costs[start] = 0;

    while let Some(Reverse(NodeCost { node, cost })) = queue.pop() {
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

        if cost > costs[node] { continue; }

        for edge in &graph.adjacencies[node] {
            let new_cost = cost + edge.weight;
            
            if new_cost < costs[edge.node] {
                queue.push(Reverse(NodeCost { cost: new_cost, node: edge.node }));
                costs[edge.node] = new_cost;
                previous[edge.node] = node;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    // This is the directed graph we're going to use.
    //
    //                7
    //        +-----------------+
    //        |                 |
    //        v   1        2    |  2
    //        0 -----> 1 -----> 3 ---> 4
    //        |        ^        ^      ^
    //        |        | 1      |      |
    //        |        |        | 3    | 1
    //        +------> 2 -------+      |
    //         10      |               |
    //                 +---------------+
    //

    #[test]
    fn check_distance_to() {
        let mut g = super::Graph::new(5);
        g.add_edge(0, 2, 10);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 3, 2);
        g.add_edge(2, 1, 1);
        g.add_edge(2, 3, 3);
        g.add_edge(2, 4, 1);
        g.add_edge(3, 0, 7);
        g.add_edge(3, 4, 2);

        assert_eq!(super::distance_to(&g, 0, 1), Some(1));
        assert_eq!(super::distance_to(&g, 0, 3), Some(3));
        assert_eq!(super::distance_to(&g, 3, 0), Some(7));
        assert_eq!(super::distance_to(&g, 0, 4), Some(5));
        assert_eq!(super::distance_to(&g, 4, 0), None);
    }

    #[test]
    fn check_shortest_path_to() {
        let mut g = super::Graph::new(5);
        g.add_edge(0, 2, 10);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 3, 2);
        g.add_edge(2, 1, 1);
        g.add_edge(2, 3, 3);
        g.add_edge(2, 4, 1);
        g.add_edge(3, 0, 7);
        g.add_edge(3, 4, 2);

        assert_eq!(super::shortest_path_to(&g, 0, 1), Some(vec![0, 1]));
        assert_eq!(super::shortest_path_to(&g, 0, 3), Some(vec![0, 1, 3]));
        assert_eq!(super::shortest_path_to(&g, 3, 0), Some(vec![3, 0]));
        assert_eq!(super::shortest_path_to(&g, 0, 4), Some(vec![0, 1, 3, 4]));
        assert_eq!(super::shortest_path_to(&g, 4, 0), None);
    }
}