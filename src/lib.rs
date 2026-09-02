#![recursion_limit="2048"]
mod app;
mod components;
mod logging;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    logging::init_logger();
    yew::Renderer::<app::App>::new().render();

    Ok(())
}
