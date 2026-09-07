use crate::app::{App, Msg, ViewMode};
use yew::{html, Context, Html};
use web_sys::InputEvent;
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

const SCALE: u32 = 3;

pub fn get_canvas(canvas_id: &str) -> (web_sys::HtmlCanvasElement, web_sys::CanvasRenderingContext2d) {
    let canvas = document().get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

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

    let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels), DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
    context2.put_image_data(&img_data, 0.0, 0.0 ).expect("Couldn't put image data into the scaling canvas");

    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.draw_image_with_html_canvas_element_and_dw_and_dh(&canvas2, 0.0, 0.0, canvas.width() as f64, canvas.height() as f64).expect("Couldn't draw the scaled canvas");
}

pub fn clear_canvas() {
    let (canvas, context) = get_canvas("gba-canvas");
    let (canvas2, _) = get_canvas("gba-canvas2");

    canvas2.set_width(DISPLAY_WIDTH);
    canvas2.set_height(DISPLAY_HEIGHT);
    canvas.set_width(DISPLAY_WIDTH * SCALE);
    canvas.set_height(DISPLAY_HEIGHT * SCALE);

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
    pub fn view_canvas(&self, ctx: &Context<Self>) -> Html {
        let is_3d = self.view_mode != ViewMode::Flat;

        let option = |mode: ViewMode| -> Html {
            let active = if self.view_mode == mode { "view-option active" } else { "view-option" };
            html! {
                <button class={active} onclick={ctx.link().callback(move |_| Msg::SetViewMode(mode))}>
                    {mode.label()}
                </button>
            }
        };

        html! {
            <div class="gba-stage">
                <div class="gba-body"
                     data-mode={self.view_mode.slug()}
                     data-spin={if self.auto_spin { "on" } else { "off" }}
                     data-lid={self.lid_angle.to_string()}
                     data-reset={self.reset_nonce.to_string()}>
                    <canvas id="gba-3d" class="gba-3d-canvas"></canvas>
                    <canvas id="gba-canvas" class="app-canvas"></canvas>
                    <canvas id="gba-canvas2" style="display:none;"></canvas>
                </div>

                <div class="view-panel">
                    <div class="view-switch">
                        {option(ViewMode::Flat)}
                        {option(ViewMode::Gba)}
                        {option(ViewMode::GbaSp)}
                    </div>

                    <div class="view-tools" hidden={!is_3d}>
                        <button class="view-tool" onclick={ctx.link().callback(|_| Msg::ResetView)}>
                            {"Reset view"}
                        </button>
                        <button class="view-tool" onclick={ctx.link().callback(|_| Msg::ToggleAutoSpin)}>
                            {if self.auto_spin { "Auto-spin: on" } else { "Auto-spin: off" }}
                        </button>
                        <label class="view-slider" hidden={self.view_mode != ViewMode::GbaSp}>
                            <span>{"Lid"}</span>
                            <input type="range" min="0" max="150" value={self.lid_angle.to_string()}
                                oninput={ctx.link().callback(|e: InputEvent| Msg::SetLidAngle(crate::dom_util::input_value(&e)))}/>
                        </label>
                    </div>

                    <p class="view-hint" hidden={!is_3d}>
                        {"Drag to rotate · Scroll to zoom · Shift-drag to pan · Double-click to reset"}
                    </p>
                </div>
            </div>
        }
    }
}