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

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    id_to_index: HashMap<u32, usize>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        let id_to_index = nodes.iter()
            .enumerate()
            .map(|(i, node)| (node.id, i))
            .collect();
        Self { nodes, edges, id_to_index }
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
        self.edges.iter()
            .filter_map(|e| {
                if e.source == node_id {
                    Some(e.target)
                }  else if e.target == node_id {
                    Some(e.source)
                } else {
                    None
                }
            })
            .collect()
    }
}