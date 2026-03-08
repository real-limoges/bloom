use crate::graph::Graph;
use glam::Vec2;

pub struct ForceParams {
    pub attraction: f32,
    pub repulsion: f32,
    pub gravity: f32,
    pub damping: f32,
}

impl Default for ForceParams {
    fn default() -> Self {
        Self { attraction: 0.1, repulsion: 100.0, gravity: 0.01, damping: 0.9 }
    }
}

pub struct ForceLayout {
    pub params: ForceParams,
    velocities: Vec<Vec2>
}

impl ForceLayout {
    pub fn new(node_count: usize, params: ForceParams) -> Self {
        Self { params, velocities: vec![Vec2::ZERO; node_count] }
    }

    pub fn step(&self, graph: &mut Graph) {
        let nodes = graph.nodes_mut();
        let mut forces = vec![Vec2::ZERO; nodes.len()];

        // repulsion
        // for i in 0..nodes.len() {
        //     for j in i + 1..nodes.len() {
        //         let pi = Vec2::new(nodes[i].x, nodes[i].y);
        //         let pj = Vec2::new(nodes[j].x, nodes[j].y);
        //         let delta = pi -pj;
        //
        //     }
        // }

        // attraction
        for edge in graph.edges() {
            if let (Some(i) , Some(j)) = {
                graph.node_index(edge.source),
                graph.node_index(edge.target),
            } {
                let nodes = graph.nodes();
                let pi = Vec2::new(nodes[i].x, nodes[i].y);
                let pj = Vec2::new(nodes[j].x, nodes[j].y);
                let delta = pj - pi;
                let dist = delta.length().max(0.1);
                let f = delta.normalize() * (dist * self.params.attraction);
                let nodes = graph.nodes_mut();

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