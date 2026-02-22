use crate::graph::types::Graph;

/// Iterative PageRank until convergence.
///
/// d = 0.85 is the standard damping factor.
/// Returns a Vec<f32> of scores aligned with graph.nodes().
pub fn pagerank(graph: &Graph, iterations: usize, damping: f32) -> Vec<f32> {
    let n = graph.node_count();
    if n == 0 {
        return vec![];
    }

    let mut scores = vec![1.0 / n as f32; n];

    for _ in 0..iterations {
        let mut next = vec![(1.0 - damping) / n as f32; n];

        for (i, node) in graph.nodes().iter().enumerate() {
            let out_neighbors = graph.neighbors(node.id);
            if out_neighbors.is_empty() {
                // Dangling node: distribute evenly
                let share = scores[i] * damping / n as f32;
                for s in next.iter_mut() {
                    *s += share;
                }
            } else {
                let share = scores[i] * damping / out_neighbors.len() as f32;
                for neighbor_id in &out_neighbors {
                    if let Some(j) = graph.node_index(*neighbor_id) {
                        next[j] += share;
                    }
                }
            }
        }

        scores = next;
    }

    scores
}

/// Stub: Louvain community detection.
/// Returns a community ID per node (index-aligned with graph.nodes()).
pub fn louvain(_graph: &Graph) -> Vec<usize> {
    // TODO: implement Louvain modularity optimisation
    vec![]
}

/// Stub: Dijkstra shortest path.
/// Returns the node-index path from `source_id` to `target_id`, or None.
pub fn shortest_path(_graph: &Graph, _source_id: u32, _target_id: u32) -> Option<Vec<usize>> {
    // TODO: implement Dijkstra
    None
}

/// Stub: betweenness centrality.
pub fn betweenness_centrality(_graph: &Graph) -> Vec<f32> {
    // TODO: implement Brandes algorithm
    vec![]
}
