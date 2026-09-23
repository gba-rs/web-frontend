use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{Event, TargetCast};

pub fn input_value(e: &InputEvent) -> String {
    e.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

pub fn files_from_input(e: Event) -> Option<web_sys::FileList> {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.files()
}
