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
}
