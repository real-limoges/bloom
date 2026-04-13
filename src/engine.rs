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
    /// Frames remaining where layout runs at high substep count for fast
    /// initial convergence. Decremented each tick.
    warmup_frames: u32,
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
            warmup_frames: 0,
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

        // Seed on a ring instead of random scatter. Starting from a circle
        // gives the force layout a topology that converges in a handful of
        // substeps without the chaotic "ball of yarn" transient.
        let n = graph.node_count() as f32;
        let radius = (n.sqrt() * 14.0).max(40.0);
        let two_pi = std::f32::consts::TAU;
        for (i, node) in graph.nodes_mut().iter_mut().enumerate() {
            let theta = (i as f32) * two_pi / n.max(1.0);
            node.x = theta.cos() * radius;
            node.y = theta.sin() * radius;
        }

        let layout = ForceLayout::new(graph.node_count(), ForceParams::default());
        let quadtree = build_quadtree(&graph);

        // Fit the camera to the initial node cloud so content isn't a
        // cluster of specks in a huge viewport. Canvas_w/h are world units
        // covered at zoom=1, so the zoom needed to fit an AABB of size
        // (aabb_w, aabb_h) with padding P is min(cw/(aw*P), ch/(ah*P)).
        let (cx, cy, zoom) = AABB::enclosing(graph.nodes().iter().map(|n| (n.x, n.y)))
            .map(|b| {
                let (cx, cy) = b.center();
                let aw = b.width().max(1.0);
                let ah = b.height().max(1.0);
                let pad = 1.4;
                let zx = self.canvas_width / (aw * pad);
                let zy = self.canvas_height / (ah * pad);
                (cx, cy, zx.min(zy).clamp(0.1, 50.0))
            })
            .unwrap_or((0.0, 0.0, 1.0));

        self.graph = Some(graph);
        self.layout = Some(layout);
        self.quadtree = Some(quadtree);
        self.camera.snap_to(cx, cy, zoom);
        // Run aggressive multi-substep physics for ~1 second so the graph
        // visibly snaps to a reasonable layout instead of drifting in chaos.
        self.warmup_frames = 60;

        Ok(())
    }

    pub fn tick(&mut self, dt: f32) {
        if let (Some(graph), Some(layout)) = (&mut self.graph, &mut self.layout) {
            // During warm-up run multiple physics substeps per rendered
            // frame so the layout converges in roughly one second instead of
            // five. After the warm-up budget is spent, fall back to a single
            // step so interactive panning stays cheap.
            let substeps = if self.warmup_frames > 0 { 4 } else { 1 };
            for _ in 0..substeps {
                layout.step(graph);
            }
            if self.warmup_frames > 0 {
                self.warmup_frames -= 1;
            }
            self.quadtree = Some(build_quadtree(graph));
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
                                    // Deep purple-black, matching the site's
                                    // synthwave dark base-300.
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.035,
                                        g: 0.020,
                                        b: 0.055,
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

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
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

        // Reset camera so the hit test is independent of load_graph's auto-fit.
        engine.camera_mut().snap_to(0.0, 0.0, 1.0);
        engine.tick(0.0);

        // Screen center (400, 300) maps to world origin at zoom 1, so node 0
        // at (0, 0) should be hit.
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
