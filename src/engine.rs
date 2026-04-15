use crate::graph::{AABB, Graph, Node, Quadtree};
use crate::layout::{ForceLayout, ForceParams};
use crate::protocol::decode::Decoder;
use crate::render::backend::RenderBackend;
use crate::render::camera::Camera;
use crate::render::edges::EdgeRenderer;
use crate::render::nodes::NodeRenderer;
use crate::render::text::TextRenderer;

pub struct BloomEngine {
    graph: Option<Graph>,
    layout: Option<ForceLayout>,
    camera: Camera,
    quadtree: Option<Quadtree>,
    canvas_width: f32,
    canvas_height: f32,
    backend: Option<RenderBackend>,
    node_renderer: Option<NodeRenderer>,
    edge_renderer: Option<EdgeRenderer>,
    text_renderer: Option<TextRenderer>,
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
            backend: None,
            node_renderer: None,
            edge_renderer: None,
            text_renderer: None,
        }
    }

    pub async fn init_renderer(
        &mut self,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), String> {
        let backend = RenderBackend::new(canvas).await?;
        let format = backend.format();
        let node_renderer = NodeRenderer::new(&backend.device, format);
        let edge_renderer = EdgeRenderer::new(&backend.device, format);
        let text_renderer = TextRenderer::new(&backend.device, &backend.queue, format);
        self.backend = Some(backend);
        self.node_renderer = Some(node_renderer);
        self.edge_renderer = Some(edge_renderer);
        self.text_renderer = Some(text_renderer);
        Ok(())
    }

    pub fn load_graph(&mut self, data: &[u8]) -> Result<(), String> {
        let mut decoder = Decoder::new(data);
        let mut graph = decoder.decode_graph()?;

        // Randomize initial positions with deterministic LCG (seed=42)
        let radius = (graph.node_count() as f32).sqrt() * 10.0;
        graph.nodes_mut().iter_mut().fold(42u32, |lcg, node| {
            let (lcg, rx) = lcg_next(lcg);
            let (lcg, ry) = lcg_next(lcg);
            node.x = rx * radius;
            node.y = ry * radius;
            lcg
        });

        let layout = ForceLayout::new(graph.node_count(), ForceParams::default());
        let quadtree = build_quadtree(&graph);

        if let Some((min_x, min_y, max_x, max_y)) = percentile_bounds(graph.nodes(), 0.9) {
            self.camera.fit_to_bounds(
                min_x,
                min_y,
                max_x,
                max_y,
                self.canvas_width,
                self.canvas_height,
                0.1,
            );
        } else {
            self.camera.focus_on(0.0, 0.0, 1.0);
        }

        self.graph = Some(graph);
        self.layout = Some(layout);
        self.quadtree = Some(quadtree);

        Ok(())
    }

    pub fn tick(&mut self, dt: f32) {
        if let (Some(graph), Some(layout)) = (&mut self.graph, &mut self.layout) {
            layout.step(graph);
            self.quadtree = Some(build_quadtree(graph));

            if let Some((min_x, min_y, max_x, max_y)) = percentile_bounds(graph.nodes(), 0.9) {
                self.camera.fit_to_bounds(
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                    self.canvas_width,
                    self.canvas_height,
                    0.1,
                );
            }
        }
        self.camera.update(dt);

        // Render if backend is initialized
        if let (Some(backend), Some(node_renderer), Some(edge_renderer)) = (
            &self.backend,
            &mut self.node_renderer,
            &mut self.edge_renderer,
        ) {
            if let Some(graph) = &self.graph {
                node_renderer.update(
                    &backend.device,
                    &backend.queue,
                    graph.nodes(),
                    &self.camera,
                    self.canvas_width,
                    self.canvas_height,
                );
                edge_renderer.update(
                    &backend.device,
                    &backend.queue,
                    graph,
                    &self.camera,
                    self.canvas_width,
                    self.canvas_height,
                );
                if let Some(text_renderer) = &mut self.text_renderer {
                    text_renderer.update(
                        &backend.device,
                        &backend.queue,
                        graph.nodes(),
                        &self.camera,
                        self.canvas_width,
                        self.canvas_height,
                    );
                }
            }

            match backend.begin_frame() {
                Ok((frame, view, mut encoder)) => {
                    {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("bloom_render_pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                depth_slice: None,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.05,
                                        g: 0.05,
                                        b: 0.08,
                                        a: 1.0,
                                    }),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            ..Default::default()
                        });

                        edge_renderer.draw(&mut pass);
                        node_renderer.draw(&mut pass);
                        if let Some(text_renderer) = &self.text_renderer {
                            text_renderer.draw(&mut pass);
                        }
                    }
                    backend.end_frame(encoder, frame);
                }
                Err(e) => {
                    log::warn!("Frame dropped: {}", e);
                }
            }
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.canvas_width = width;
        self.canvas_height = height;
        if let Some(backend) = &mut self.backend {
            backend.resize(width as u32, height as u32);
        }
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
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, node)| node)
    }

    pub fn focus_node(&mut self, node_id: u32) {
        if let Some(graph) = &self.graph
            && let Some(node) = graph.node_by_id(node_id)
        {
            self.camera.focus_on(node.x, node.y, 2.0);
        }
    }

    pub fn graph(&self) -> Option<&Graph> {
        self.graph.as_ref()
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Flat [x0, y0, x1, y1, ...] in canvas pixel coordinates for every node,
    /// computed against the current camera. Empty if no graph is loaded.
    /// Hot path for overlay renderers; avoid per-frame allocation churn downstream.
    pub fn node_screen_positions(&self) -> Vec<f32> {
        let Some(graph) = &self.graph else {
            return Vec::new();
        };
        let w = self.canvas_width as f64;
        let h = self.canvas_height as f64;
        let nodes = graph.nodes();
        let mut out = Vec::with_capacity(nodes.len() * 2);
        for node in nodes {
            let (sx, sy) = self.camera.world_to_screen(node.x, node.y, w, h);
            out.push(sx as f32);
            out.push(sy as f32);
        }
        out
    }
}

fn lcg_next(state: u32) -> (u32, f32) {
    let state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    (state, (state as f32 / u32::MAX as f32) * 2.0 - 1.0)
}

/// AABB of the middle `percentile` fraction of nodes per axis. Robust to outliers
/// that drift far from the main cluster — fits the camera to the dense core
/// instead of letting one or two stragglers dominate the frame.
fn percentile_bounds(nodes: &[Node], percentile: f32) -> Option<(f32, f32, f32, f32)> {
    if nodes.is_empty() {
        return None;
    }
    let n = nodes.len();
    let mut xs: Vec<f32> = nodes.iter().map(|node| node.x).collect();
    let mut ys: Vec<f32> = nodes.iter().map(|node| node.y).collect();
    xs.sort_by(|a, b| a.total_cmp(b));
    ys.sort_by(|a, b| a.total_cmp(b));
    let trim = (((n as f32) * (1.0 - percentile) * 0.5) as usize).min(n / 2);
    let lo = trim;
    let hi = (n - 1).saturating_sub(trim);
    Some((xs[lo], ys[lo], xs[hi], ys[hi]))
}

fn build_quadtree(graph: &Graph) -> Quadtree {
    let nodes = graph.nodes();
    let default_bounds = AABB {
        min_x: -100.0,
        min_y: -100.0,
        max_x: 100.0,
        max_y: 100.0,
    };
    let bounds = AABB::enclosing(nodes.iter().map(|n| (n.x, n.y)))
        .map(|b| b.padded(0.05))
        .unwrap_or(default_bounds);

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
    fn screen_positions_use_camera_and_canvas() {
        let nodes = &[(1, 0.0f32, 0u16), (2, 0.0, 0)];
        let data = build_blom(nodes, &[], None);

        let mut engine = BloomEngine::new(800.0, 600.0);
        engine.load_graph(&data).unwrap();

        engine.graph.as_mut().unwrap().nodes_mut()[0].x = 0.0;
        engine.graph.as_mut().unwrap().nodes_mut()[0].y = 0.0;
        engine.graph.as_mut().unwrap().nodes_mut()[1].x = 100.0;
        engine.graph.as_mut().unwrap().nodes_mut()[1].y = 0.0;

        // Run the camera toward (0, 0) zoom 1 — load_graph's focus_on is a target,
        // not an instant snap.
        for _ in 0..200 {
            engine.camera.update(0.016);
        }

        let positions = engine.node_screen_positions();
        assert_eq!(positions.len(), 4);
        assert!(
            (positions[0] - 400.0).abs() < 1e-2,
            "node0 x not at canvas center: {}",
            positions[0]
        );
        assert!(
            (positions[1] - 300.0).abs() < 1e-2,
            "node0 y not at canvas center: {}",
            positions[1]
        );
        assert!(
            (positions[2] - 500.0).abs() < 1e-2,
            "node1 x off: {}",
            positions[2]
        );
        assert!(
            (positions[3] - 300.0).abs() < 1e-2,
            "node1 y off: {}",
            positions[3]
        );
    }

    #[test]
    fn screen_positions_empty_when_no_graph() {
        let engine = BloomEngine::new(800.0, 600.0);
        assert!(engine.node_screen_positions().is_empty());
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
