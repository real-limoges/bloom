use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub label: String,
    pub pagerank: f32,
    pub degree: u16,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    id_to_index: HashMap<u32, usize>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        let id_to_index = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (node.id, i))
            .collect();
        Self {
            nodes,
            edges,
            id_to_index,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    pub fn nodes_mut(&mut self) -> &mut [Node] {
        &mut self.nodes
    }
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }
    pub fn node_by_id(&self, id: u32) -> Option<&Node> {
        self.id_to_index.get(&id).map(|&i| &self.nodes[i])
    }
    pub fn node_index(&self, id: u32) -> Option<usize> {
        self.id_to_index.get(&id).copied()
    }
    /// Undirected neighbors: every node connected by an edge in either direction.
    pub fn neighbors(&self, node_id: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter_map(|e| {
                if e.source == node_id {
                    Some(e.target)
                } else if e.target == node_id {
                    Some(e.source)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Directed out-neighbors: targets of edges originating at `node_id`.
    pub fn out_neighbors(&self, node_id: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|e| e.source == node_id)
            .map(|e| e.target)
            .collect()
    }

    /// Directed out-neighbor adjacency as CSR. `offsets` has length `node_count + 1`;
    /// `neighbors[offsets[i]..offsets[i+1]]` are the graph-indices (not ids) of the
    /// out-neighbors of node i. Intended for random-walk consumers on the JS side,
    /// where per-step allocation would be a hot-path cost.
    pub fn out_adjacency_csr(&self) -> (Vec<u32>, Vec<u32>) {
        let n = self.nodes.len();
        let mut counts = vec![0u32; n];
        for edge in &self.edges {
            if let (Some(&src_idx), Some(_)) = (
                self.id_to_index.get(&edge.source),
                self.id_to_index.get(&edge.target),
            ) {
                counts[src_idx] += 1;
            }
        }

        let mut offsets = Vec::with_capacity(n + 1);
        offsets.push(0u32);
        let mut running = 0u32;
        for c in &counts {
            running += *c;
            offsets.push(running);
        }

        let mut write_pos: Vec<u32> = offsets[..n].to_vec();
        let mut neighbors = vec![0u32; running as usize];
        for edge in &self.edges {
            if let (Some(&src_idx), Some(&tgt_idx)) = (
                self.id_to_index.get(&edge.source),
                self.id_to_index.get(&edge.target),
            ) {
                let slot = write_pos[src_idx] as usize;
                neighbors[slot] = tgt_idx as u32;
                write_pos[src_idx] += 1;
            }
        }

        (offsets, neighbors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn sample_graph() -> Graph {
        let nodes = vec![make_node(10), make_node(20), make_node(30)];
        let edges = vec![
            Edge {
                source: 10,
                target: 20,
            },
            Edge {
                source: 20,
                target: 30,
            },
        ];
        Graph::new(nodes, edges)
    }

    #[test]
    fn counts() {
        let g = sample_graph();
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
    }

    #[test]
    fn id_to_index_lookup() {
        let g = sample_graph();
        assert_eq!(g.node_index(10), Some(0));
        assert_eq!(g.node_index(20), Some(1));
        assert_eq!(g.node_index(30), Some(2));
        assert_eq!(g.node_index(99), None);
    }

    #[test]
    fn node_by_id() {
        let g = sample_graph();
        assert_eq!(g.node_by_id(20).unwrap().id, 20);
        assert!(g.node_by_id(99).is_none());
    }

    #[test]
    fn neighbors_undirected() {
        let g = sample_graph();
        let mut n = g.neighbors(20);
        n.sort();
        assert_eq!(n, vec![10, 30]);
    }

    #[test]
    fn neighbors_leaf() {
        let g = sample_graph();
        assert_eq!(g.neighbors(10), vec![20]);
        assert_eq!(g.neighbors(30), vec![20]);
    }

    #[test]
    fn neighbors_missing_node() {
        let g = sample_graph();
        assert!(g.neighbors(99).is_empty());
    }

    #[test]
    fn out_neighbors_respects_direction() {
        // sample_graph: 10 -> 20, 20 -> 30
        let g = sample_graph();
        assert_eq!(g.out_neighbors(10), vec![20]);
        assert_eq!(g.out_neighbors(20), vec![30]);
        assert!(g.out_neighbors(30).is_empty(), "sink has no out-neighbors");
        assert!(g.out_neighbors(99).is_empty(), "missing id yields empty");
    }

    #[test]
    fn empty_graph() {
        let g = Graph::new(vec![], vec![]);
        assert_eq!(g.node_count(), 0);
        assert_eq!(g.edge_count(), 0);
    }

    #[test]
    fn nodes_mut_updates_positions() {
        let mut g = sample_graph();
        g.nodes_mut()[0].x = 5.0;
        g.nodes_mut()[0].y = 10.0;
        assert_eq!(g.nodes()[0].x, 5.0);
        assert_eq!(g.nodes()[0].y, 10.0);
    }

    #[test]
    fn out_csr_directed_and_sinks() {
        // 10 -> 20, 10 -> 30, 20 -> 30, 30 has no out-edges.
        let nodes = vec![make_node(10), make_node(20), make_node(30)];
        let edges = vec![
            Edge {
                source: 10,
                target: 20,
            },
            Edge {
                source: 10,
                target: 30,
            },
            Edge {
                source: 20,
                target: 30,
            },
        ];
        let g = Graph::new(nodes, edges);
        let (offsets, neighbors) = g.out_adjacency_csr();

        assert_eq!(offsets, vec![0, 2, 3, 3]);
        // Indices, not ids: node 10 -> index 0, node 20 -> 1, node 30 -> 2
        let n10: Vec<u32> = neighbors[offsets[0] as usize..offsets[1] as usize].to_vec();
        let mut n10_sorted = n10.clone();
        n10_sorted.sort();
        assert_eq!(n10_sorted, vec![1, 2]);
        assert_eq!(&neighbors[offsets[1] as usize..offsets[2] as usize], &[2]);
        assert_eq!(
            &neighbors[offsets[2] as usize..offsets[3] as usize],
            &[] as &[u32]
        );
    }

    #[test]
    fn out_csr_empty_graph() {
        let g = Graph::new(vec![], vec![]);
        let (offsets, neighbors) = g.out_adjacency_csr();
        assert_eq!(offsets, vec![0]);
        assert!(neighbors.is_empty());
    }

    #[test]
    fn out_csr_ignores_dangling_edges() {
        // Edge references an id not in the node set — should be silently dropped.
        let nodes = vec![make_node(10), make_node(20)];
        let edges = vec![
            Edge {
                source: 10,
                target: 20,
            },
            Edge {
                source: 10,
                target: 999,
            },
            Edge {
                source: 999,
                target: 20,
            },
        ];
        let g = Graph::new(nodes, edges);
        let (offsets, neighbors) = g.out_adjacency_csr();
        assert_eq!(offsets, vec![0, 1, 1]);
        assert_eq!(neighbors, vec![1]);
    }
}
