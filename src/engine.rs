use crate::graph::{Graph, Node, Quadtree, AABB};
use crate::layout::{ForceLayout, ForceParams};
use crate::protocol::decode::Decoder;
use crate::render::camera::Camera;

pub struct BloomEngine {
    graph: Option<Graph>,
    layout: Option<ForceLayout>,
    camera: Camera,
    quadtree: Option<Quadtree>,
    canvas_width: f32,
    canvas_height: f32,
}

impl BloomEngine {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            graph: None,
            layout: None,
            camera: Camera::new(),
            quadtree: None,
            canvas_width: width,
            canvas_height: height,
        }
    }

    pub fn load_graph(&mut self, data: &[u8]) -> Result<(), String> {
        let mut decoder = Decoder::new(data);
        let mut graph = decoder.decode_graph()?;

        // Randomize initial positions with deterministic LCG (seed=42)
        let n = graph.node_count();
        let radius = (n as f32).sqrt() * 10.0;
        let mut lcg = 42u32;
        for node in graph.nodes_mut() {
            lcg = lcg.wrapping_mul(1664525).wrapping_add(1013904223);
            let rx = (lcg as f32 / u32::MAX as f32) * 2.0 - 1.0;
            lcg = lcg.wrapping_mul(1664525).wrapping_add(1013904223);
            let ry = (lcg as f32 / u32::MAX as f32) * 2.0 - 1.0;
            node.x = rx * radius;
            node.y = ry * radius;
        }

        let layout = ForceLayout::new(n, ForceParams::default());
        let quadtree = build_quadtree(&graph);

        self.graph = Some(graph);
        self.layout = Some(layout);
        self.quadtree = Some(quadtree);
        self.camera.focus_on(0.0, 0.0, 1.0);

        Ok(())
    }

    pub fn tick(&mut self, dt: f32) {
        if let (Some(graph), Some(layout)) = (&mut self.graph, &mut self.layout) {
            layout.step(graph);
            self.quadtree = Some(build_quadtree(graph));
        }
        self.camera.update(dt);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas_width = width;
        self.canvas_height = height;
    }

    pub fn node_at(&self, screen_x: f32, screen_y: f32) -> Option<&Node> {
        let graph = self.graph.as_ref()?;
        let quadtree = self.quadtree.as_ref()?;

        let (wx, wy) = self.camera.screen_to_world(
            screen_x as f64,
            screen_y as f64,
            self.canvas_width as f64,
            self.canvas_height as f64,
        );

        let hit_radius = 10.0 / self.camera.zoom;
        let candidates = quadtree.query_point(wx, wy, hit_radius);

        let nodes = graph.nodes();
        candidates
            .iter()
            .filter_map(|&idx| {
                let node = &nodes[idx];
                let dx = node.x - wx;
                let dy = node.y - wy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= hit_radius {
                    Some((dist, node))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|(_, node)| node)
    }

    pub fn focus_node(&mut self, node_id: u32) {
        if let Some(graph) = &self.graph {
            if let Some(node) = graph.node_by_id(node_id) {
                self.camera.focus_on(node.x, node.y, 2.0);
            }
        }
    }

    pub fn graph(&self) -> Option<&Graph> {
        self.graph.as_ref()
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}

fn build_quadtree(graph: &Graph) -> Quadtree {
    let nodes = graph.nodes();
    if nodes.is_empty() {
        return Quadtree::new(
            AABB {
                min_x: -100.0,
                min_y: -100.0,
                max_x: 100.0,
                max_y: 100.0,
            },
            4,
        );
    }

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for node in nodes {
        min_x = min_x.min(node.x);
        min_y = min_y.min(node.y);
        max_x = max_x.max(node.x);
        max_y = max_y.max(node.y);
    }

    // 5% padding
    let pad_x = (max_x - min_x) * 0.05 + 1.0;
    let pad_y = (max_y - min_y) * 0.05 + 1.0;

    let bounds = AABB {
        min_x: min_x - pad_x,
        min_y: min_y - pad_y,
        max_x: max_x + pad_x,
        max_y: max_y + pad_y,
    };

    let mut qt = Quadtree::new(bounds, 4);
    for (i, node) in nodes.iter().enumerate() {
        qt.insert(i, node);
    }
    qt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::build_blom;

    #[test]
    fn load_graph_populates_state() {
        let nodes = &[(1, 0.1f32, 2u16), (2, 0.2, 3), (3, 0.3, 1)];
        let edges = &[(1u32, 2u32), (2, 3)];
        let data = build_blom(nodes, edges, None);

        let mut engine = BloomEngine::new(800.0, 600.0);
        engine.load_graph(&data).unwrap();

        let graph = engine.graph().unwrap();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);

        // Positions should be randomized (not all zero)
        let any_nonzero = graph.nodes().iter().any(|n| n.x != 0.0 || n.y != 0.0);
        assert!(any_nonzero, "positions should be randomized");
    }

    #[test]
    fn tick_advances_positions() {
        let nodes = &[(1, 0.1f32, 1u16), (2, 0.2, 1)];
        let edges = &[(1u32, 2u32)];
        let data = build_blom(nodes, edges, None);

        let mut engine = BloomEngine::new(800.0, 600.0);
        engine.load_graph(&data).unwrap();

        let before: Vec<(f32, f32)> = engine
            .graph()
            .unwrap()
            .nodes()
            .iter()
            .map(|n| (n.x, n.y))
            .collect();

        engine.tick(0.016);

        let after: Vec<(f32, f32)> = engine
            .graph()
            .unwrap()
            .nodes()
            .iter()
            .map(|n| (n.x, n.y))
            .collect();

        assert_ne!(before, after, "positions should change after tick");
    }

    #[test]
    fn node_at_hit_test() {
        let nodes = &[(1, 0.0f32, 0u16), (2, 0.0, 0)];
        let data = build_blom(nodes, &[], None);

        let mut engine = BloomEngine::new(800.0, 600.0);
        engine.load_graph(&data).unwrap();

        // Place node 0 at a known position
        engine.graph.as_mut().unwrap().nodes_mut()[0].x = 0.0;
        engine.graph.as_mut().unwrap().nodes_mut()[0].y = 0.0;
        engine.graph.as_mut().unwrap().nodes_mut()[1].x = 100.0;
        engine.graph.as_mut().unwrap().nodes_mut()[1].y = 100.0;

        // Rebuild quadtree with new positions
        engine.tick(0.0);

        // Screen center maps to world origin (camera at 0,0 zoom 1)
        // After tick(0.0), camera hasn't moved much from focus_on(0,0,1)
        // Screen center = (400, 300), which maps to world ~(0,0)
        let hit = engine.node_at(400.0, 300.0);
        assert!(hit.is_some(), "should hit node near origin");
    }

    #[test]
    fn load_graph_replaces_previous() {
        let data1 = build_blom(&[(1, 0.0, 0), (2, 0.0, 0)], &[], None);
        let data2 = build_blom(&[(10, 0.0, 0), (20, 0.0, 0), (30, 0.0, 0)], &[], None);

        let mut engine = BloomEngine::new(800.0, 600.0);
        engine.load_graph(&data1).unwrap();
        assert_eq!(engine.graph().unwrap().node_count(), 2);

        engine.load_graph(&data2).unwrap();
        assert_eq!(engine.graph().unwrap().node_count(), 3);
    }
}
