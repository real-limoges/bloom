use crate::graph::Graph;
use crate::layout::barnes_hut::BarnesHutTree;
use glam::Vec2;

pub struct ForceParams {
    pub attraction: f32,
    pub repulsion: f32,
    pub gravity: f32,
    pub damping: f32,
    pub theta: f32,
    /// Cap on per-step displacement (world units). Without it, attraction
    /// scales linearly with distance while repulsion only falls off as 1/r²,
    /// so any transient that knocks two clusters apart blows positions up
    /// to 1e18 within a few frames.
    pub max_step: f32,
}

impl Default for ForceParams {
    fn default() -> Self {
        Self {
            attraction: 0.1,
            repulsion: 100.0,
            gravity: 0.01,
            damping: 0.9,
            theta: 0.7,
            max_step: 5.0,
        }
    }
}

pub struct ForceLayout {
    pub params: ForceParams,
    velocities: Vec<Vec2>,
    /// Cooling temperature, d3-force style. Starts at 1.0 and decays
    /// geometrically toward `alpha_min`; all forces are scaled by alpha each
    /// step. Without this the system never settles — max_step just caps how
    /// far nodes get flung per frame, but they keep getting flung forever.
    alpha: f32,
    alpha_decay: f32,
    alpha_min: f32,
}

impl ForceLayout {
    pub fn new(node_count: usize, params: ForceParams) -> Self {
        Self {
            params,
            velocities: vec![Vec2::ZERO; node_count],
            alpha: 1.0,
            // 1 - 0.001^(1/300): alpha reaches alpha_min in ~300 ticks (~5s @ 60fps)
            alpha_decay: 0.0228,
            alpha_min: 0.001,
        }
    }

    /// Re-heat the simulation. Call after a structural change (graph edit,
    /// user drag) to let the layout converge to a new equilibrium.
    pub fn reheat(&mut self) {
        self.alpha = 1.0;
    }

    pub fn step(&mut self, graph: &mut Graph) {
        if self.alpha < self.alpha_min {
            return;
        }

        let nodes = graph.nodes_mut();
        let mut forces = vec![Vec2::ZERO; nodes.len()];

        // repulsion via Barnes-Hut
        let tree = BarnesHutTree::build(nodes);
        for (i, force) in forces.iter_mut().enumerate() {
            *force += tree.compute_repulsion(i, nodes, self.params.repulsion, self.params.theta);
        }

        // attraction (iterate edges directly — all borrows are immutable)
        for edge in graph.edges() {
            if let (Some(i), Some(j)) =
                (graph.node_index(edge.source), graph.node_index(edge.target))
            {
                if i == j {
                    continue;
                }
                let nodes = graph.nodes();
                let delta = Vec2::new(nodes[j].x - nodes[i].x, nodes[j].y - nodes[i].y);
                let dist = delta.length();
                // Skip coincident nodes — `delta.normalize()` returns NaN for a
                // zero vector and one bad edge poisons every position in one tick.
                if dist < 1e-6 {
                    continue;
                }
                let f = delta / dist * (dist.max(0.1) * self.params.attraction);
                forces[i] += f;
                forces[j] -= f;
            }
        }

        // gravity
        graph.nodes().iter().enumerate().for_each(|(i, node)| {
            forces[i] -= Vec2::new(node.x, node.y) * self.params.gravity;
        });

        // integrate. Defensive: drop any non-finite force component so a single
        // pathological subtree can't poison every node's position in one tick.
        let alpha = self.alpha;
        for (i, node) in graph.nodes_mut().iter_mut().enumerate() {
            let f = forces[i];
            let f = Vec2::new(
                if f.x.is_finite() { f.x } else { 0.0 },
                if f.y.is_finite() { f.y } else { 0.0 },
            ) * alpha;
            let mut v = (self.velocities[i] + f) * self.params.damping;
            let speed = v.length();
            if speed > self.params.max_step {
                v *= self.params.max_step / speed;
            }
            self.velocities[i] = v;
            node.x += v.x;
            node.y += v.y;
        }

        self.alpha += (0.0 - self.alpha) * self.alpha_decay;
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

    #[test]
    fn self_loop_does_not_nan() {
        // Repro for the all-NaN bug seen in /graph: a single self-loop edge
        // (source == target) made `delta.normalize()` return NaN, which
        // poisoned every position in one step.
        let nodes: Vec<Node> = (0..3)
            .map(|i| {
                let mut n = make_node(i);
                n.x = (i as f32) * 5.0;
                n.y = (i as f32) * 3.0;
                n
            })
            .collect();
        let edges = vec![
            Edge { source: 0, target: 0 }, // self-loop
            Edge { source: 0, target: 1 },
            Edge { source: 1, target: 2 },
        ];
        let mut graph = Graph::new(nodes, edges);
        let mut layout = ForceLayout::new(3, ForceParams::default());

        for _ in 0..10 {
            layout.step(&mut graph);
            for n in graph.nodes() {
                assert!(n.x.is_finite() && n.y.is_finite(), "NaN escaped: {:?}", (n.x, n.y));
            }
        }
    }

    #[test]
    fn layout_stays_bounded_for_realistic_size() {
        // Repro for the explosion seen in /graph — 203 nodes, ~ring of edges
        // plus one cross-link cluster. Without per-step displacement clamping
        // attraction grew linearly with distance and positions reached 1e18.
        let n = 203usize;
        let nodes: Vec<Node> = (0..n)
            .map(|i| {
                let mut node = make_node(i as u32);
                let theta = (i as f32) * 0.7;
                node.x = theta.cos() * 50.0;
                node.y = theta.sin() * 50.0;
                node
            })
            .collect();
        let mut edges: Vec<Edge> = (0..n)
            .map(|i| Edge { source: i as u32, target: ((i + 1) % n) as u32 })
            .collect();
        for i in 0..n {
            edges.push(Edge { source: i as u32, target: ((i + 7) % n) as u32 });
        }
        let mut graph = Graph::new(nodes, edges);
        let mut layout = ForceLayout::new(n, ForceParams::default());

        for step in 0..600 {
            layout.step(&mut graph);
            for node in graph.nodes() {
                assert!(
                    node.x.abs() < 1e6 && node.y.abs() < 1e6,
                    "position blew up at step {step}: ({}, {})",
                    node.x,
                    node.y,
                );
            }
        }
    }

    #[test]
    fn layout_converges_under_alpha_cooling() {
        // Without cooling, forces never settle and nodes keep oscillating.
        // After enough ticks the alpha decay should pin every node still.
        let n = 50usize;
        let nodes: Vec<Node> = (0..n)
            .map(|i| {
                let mut node = make_node(i as u32);
                node.x = (i as f32) * 0.3;
                node.y = (i as f32) * 0.2;
                node
            })
            .collect();
        let edges: Vec<Edge> = (0..n)
            .map(|i| Edge { source: i as u32, target: ((i + 1) % n) as u32 })
            .collect();
        let mut graph = Graph::new(nodes, edges);
        let mut layout = ForceLayout::new(n, ForceParams::default());

        // Settle past the cooling horizon.
        for _ in 0..1500 {
            layout.step(&mut graph);
        }

        // Snapshot, take more steps — positions must be effectively frozen.
        let snapshot: Vec<(f32, f32)> = graph.nodes().iter().map(|n| (n.x, n.y)).collect();
        for _ in 0..100 {
            layout.step(&mut graph);
        }
        for (i, node) in graph.nodes().iter().enumerate() {
            let dx = (node.x - snapshot[i].0).abs();
            let dy = (node.y - snapshot[i].1).abs();
            assert!(
                dx < 0.01 && dy < 0.01,
                "node {i} still moving after cooling: dx={dx} dy={dy}"
            );
        }
    }

    #[test]
    fn coincident_nodes_do_not_nan() {
        // Two nodes at exactly the same position connected by an edge —
        // `delta.length()` is zero, `normalize()` would NaN without a guard.
        let nodes: Vec<Node> = (0..2).map(make_node).collect();
        let edges = vec![Edge { source: 0, target: 1 }];
        let mut graph = Graph::new(nodes, edges);
        let mut layout = ForceLayout::new(2, ForceParams::default());

        for _ in 0..10 {
            layout.step(&mut graph);
            for n in graph.nodes() {
                assert!(n.x.is_finite() && n.y.is_finite(), "NaN escaped: {:?}", (n.x, n.y));
            }
        }
    }
}
