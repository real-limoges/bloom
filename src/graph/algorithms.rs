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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::{Edge, Node};

    fn make_node(id: u32) -> Node {
        Node {
            id,
            label: String::new(),
            pagerank: 0.0,
            degree: 0,
            x: 0.0,
            y: 0.0,
        }
    }

    fn triangle_graph() -> Graph {
        // 1 -> 2 -> 3 -> 1
        let nodes = vec![make_node(1), make_node(2), make_node(3)];
        let edges = vec![
            Edge {
                source: 1,
                target: 2,
            },
            Edge {
                source: 2,
                target: 3,
            },
            Edge {
                source: 3,
                target: 1,
            },
        ];
        Graph::new(nodes, edges)
    }

    #[test]
    fn pagerank_empty_graph() {
        let g = Graph::new(vec![], vec![]);
        assert!(pagerank(&g, 10, 0.85).is_empty());
    }

    #[test]
    fn pagerank_scores_sum_to_one() {
        let g = triangle_graph();
        let scores = pagerank(&g, 20, 0.85);
        let sum: f32 = scores.iter().sum();
        assert!(
            (sum - 1.0).abs() < 1e-4,
            "scores sum to {sum}, expected ~1.0"
        );
    }

    #[test]
    fn pagerank_symmetric_graph_equal_scores() {
        let g = triangle_graph();
        let scores = pagerank(&g, 50, 0.85);
        // Symmetric cycle => all nodes should converge to equal rank
        let expected = 1.0 / 3.0;
        for (i, &s) in scores.iter().enumerate() {
            assert!(
                (s - expected).abs() < 1e-3,
                "node {i} score {s}, expected ~{expected}"
            );
        }
    }

    #[test]
    fn pagerank_star_graph_center_ranks_higher() {
        // 1 is the hub: 2->1, 3->1, 4->1
        let nodes = vec![make_node(1), make_node(2), make_node(3), make_node(4)];
        let edges = vec![
            Edge {
                source: 2,
                target: 1,
            },
            Edge {
                source: 3,
                target: 1,
            },
            Edge {
                source: 4,
                target: 1,
            },
        ];
        let g = Graph::new(nodes, edges);
        let scores = pagerank(&g, 30, 0.85);
        // Node 1 (index 0) should have the highest score
        let max_idx = scores
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        assert_eq!(max_idx, 0, "hub node should rank highest");
    }

    #[test]
    fn stubs_return_empty() {
        let g = triangle_graph();
        assert!(louvain(&g).is_empty());
        assert!(shortest_path(&g, 1, 3).is_none());
        assert!(betweenness_centrality(&g).is_empty());
    }
}
