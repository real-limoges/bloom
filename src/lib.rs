use wasm_bindgen::prelude::*;

pub mod engine;
pub mod graph;
pub mod layout;
pub mod protocol;
pub mod render;
pub mod spatial;

#[cfg(test)]
pub mod test_utils;

#[wasm_bindgen]
pub struct BloomEngine {
    inner: engine::BloomEngine,
}

#[wasm_bindgen]
impl BloomEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<BloomEngine, JsValue> {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Bloom engine initializing");
        let width = canvas.width() as f32;
        let height = canvas.height() as f32;
        Ok(BloomEngine {
            inner: engine::BloomEngine::new(width, height),
        })
    }

    pub fn load_graph(&mut self, data: &[u8]) -> Result<(), JsValue> {
        self.inner
            .load_graph(data)
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn tick(&mut self, dt: f32) {
        self.inner.tick(dt);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.inner.resize(width, height);
    }

    pub fn hover(&self, screen_x: f32, screen_y: f32) -> Option<u32> {
        self.inner.node_at(screen_x, screen_y).map(|n| n.id)
    }

    pub fn focus_node(&mut self, node_id: u32) {
        self.inner.focus_node(node_id);
    }
}
