use crate::app::App;
use yew::{html, Html};
use web_sys::ImageData;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, Clamped};
use gba_emulator::gpu::gpu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn get_canvas(canvas_id: &str) -> (web_sys::HtmlCanvasElement, web_sys::CanvasRenderingContext2d) {
    let canvas = document().get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(DISPLAY_WIDTH);
    canvas.set_height(DISPLAY_HEIGHT);

    // TODO method that takes cavnas id to return tuple of canvas and context
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    return (canvas, context);
}

pub fn show_canvas(mut pixels: Vec<u8>) {
    let (canvas, context) = get_canvas("gba-canvas");
    let (canvas2, context2) = get_canvas("gba-canvas2");

    let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels), canvas.width(), canvas.height()).unwrap();
    context2.put_image_data(&img_data, 0.0, 0.0 ).expect("Couldn't put image data into the scaling canvas");

    let scale = 3;
    canvas.set_width(DISPLAY_WIDTH * scale);
    canvas.set_height(DISPLAY_HEIGHT * scale);
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.draw_image_with_html_canvas_element_and_dw_and_dh(&canvas2, 0.0, 0.0, (DISPLAY_WIDTH * scale) as f64, (DISPLAY_HEIGHT * scale) as f64).expect("Couldn't draw the scaled canvas");
}

pub fn clear_canvas() {
    let (canvas, context) = get_canvas("gba-canvas");
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}

pub fn convert_frame_to_u8(vec: &Vec<u32>) -> Vec<u8> {
    let mut new_vec: Vec<u8> = Vec::new();
    for i in 0..vec.len() {
        new_vec.push(((vec[i] & 0xFF_00_00) >> 16) as u8);
        new_vec.push(((vec[i] & 0xFF_00) >> 8) as u8);
        new_vec.push((vec[i] & 0xFF) as u8);
        new_vec.push(0xFF as u8);
    }
    new_vec
}

impl App {
    pub fn view_canvas(&self) -> Html {
        html! {
            <>
                <canvas id="gba-canvas"></canvas>
                <canvas id="gba-canvas2" style="display:none;"></canvas>
            </>
        }
    }
}