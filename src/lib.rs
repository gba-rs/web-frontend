#![recursion_limit="2048"]
mod app;
mod audio;
mod backgrounds_decode;
mod components;
mod dom_util;
mod frame_pacing;
mod logging;
mod route;
mod save_state;
mod storage;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    logging::init_logger();
    yew::Renderer::<app::App>::new().render();

    Ok(())
}
