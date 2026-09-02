use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};

pub fn input_value(e: &InputEvent) -> String {
    e.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}
