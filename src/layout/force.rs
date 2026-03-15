use crate::graph::Graph;
use crate::layout::barnes_hut::BarnesHutTree;
use glam::Vec2;

pub struct ForceParams {
    pub attraction: f32,
    pub repulsion: f32,
    pub gravity: f32,
    pub damping: f32,
    pub theta: f32,
}

impl Default for ForceParams {
    fn default() -> Self {
        Self {
            attraction: 0.1,
            repulsion: 100.0,
            gravity: 0.01,
            damping: 0.9,
            theta: 0.7,
        }
    }
}

pub struct ForceLayout {
    pub params: ForceParams,
    velocities: Vec<Vec2>,
}

impl ForceLayout {
    pub fn new(node_count: usize, params: ForceParams) -> Self {
        Self {
            params,
            velocities: vec![Vec2::ZERO; node_count],
        }
    }

    pub fn step(&mut self, graph: &mut Graph) {
        let nodes = graph.nodes_mut();
        let mut forces = vec![Vec2::ZERO; nodes.len()];

        // repulsion via Barnes-Hut
        let tree = BarnesHutTree::build(nodes);
        for (i, force) in forces.iter_mut().enumerate() {
            *force += tree.compute_repulsion(i, nodes, self.params.repulsion, self.params.theta);
        }

        // attraction
        let edge_pairs: Vec<(u32, u32)> =
            graph.edges().iter().map(|e| (e.source, e.target)).collect();
        for (source, target) in edge_pairs {
            if let (Some(i), Some(j)) = (graph.node_index(source), graph.node_index(target)) {
                let nodes = graph.nodes();
                let pi = Vec2::new(nodes[i].x, nodes[i].y);
                let pj = Vec2::new(nodes[j].x, nodes[j].y);
                let delta = pj - pi;
                let dist = delta.length().max(0.1);
                let f = delta.normalize() * (dist * self.params.attraction);

                forces[i] += f;
                forces[j] -= f;
            }
        }

        // gravity
        for (i, node) in graph.nodes().iter().enumerate() {
            forces[i] -= Vec2::new(node.x, node.y) * self.params.gravity;
        }

        // integrate
        for (i, node) in graph.nodes_mut().iter_mut().enumerate() {
            self.velocities[i] = (self.velocities[i] + forces[i]) * self.params.damping;
            node.x += self.velocities[i].x;
            node.y += self.velocities[i].y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Edge, Node};

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

    #[test]
    fn layout_spreads_nodes() {
        let nodes: Vec<Node> = (0..5)
            .map(|i| {
                let mut n = make_node(i);
                // Small distinct offsets so forces aren't degenerate
                n.x = (i as f32) * 0.1;
                n.y = (i as f32) * 0.07;
                n
            })
            .collect();
        let edges = vec![
            Edge {
                source: 0,
                target: 1,
            },
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
                target: 4,
            },
        ];
        let mut graph = Graph::new(nodes, edges);
        let mut layout = ForceLayout::new(5, ForceParams::default());

        for _ in 0..50 {
            layout.step(&mut graph);
        }

        let avg_dist: f32 = graph
            .nodes()
            .iter()
            .map(|n| (n.x * n.x + n.y * n.y).sqrt())
            .sum::<f32>()
            / graph.node_count() as f32;

        assert!(
            avg_dist > 1.0,
            "Nodes should have spread apart, avg distance from origin: {}",
            avg_dist
        );
    }
}
