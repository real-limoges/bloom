use wasm_bindgen::prelude::*;

pub mod engine;
pub mod graph;
pub mod layout;
pub mod protocol;
pub mod render;

#[wasm_bindgen]
pub struct BloomEngine {
    // Will be populated in Phase 4
}

#[wasm_bindgen]
impl BloomEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(_canvas: web_sys::HtmlCanvasElement) -> Result<BloomEngine, JsValue> {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Bloom engine initializing");
        Ok(BloomEngine {})
    }
}
