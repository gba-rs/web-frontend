use yew::prelude::*;
use yew::{html, Component, Context, Html, NodeRef};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;

use crate::backgrounds_decode::{decode_background, GBA_HEIGHT, GBA_WIDTH};

const THUMB_W: usize = 120;
const THUMB_H: usize = 80;

pub struct BackgroundsPanel {
    canvas_refs: [NodeRef; 4],
    props: BackgroundsPanelProp,
}

#[derive(Properties, Clone)]
pub struct BackgroundsPanelProp {
    pub gba: Rc<RefCell<GBA>>,
}

impl PartialEq for BackgroundsPanelProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba)
    }
}

impl Component for BackgroundsPanel {
    type Message = ();
    type Properties = BackgroundsPanelProp;

    fn create(ctx: &Context<Self>) -> Self {
        BackgroundsPanel {
            canvas_refs: [NodeRef::default(), NodeRef::default(), NodeRef::default(), NodeRef::default()],
            props: ctx.props().clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                { for (0..4).map(|bg_number| html! {
                    <div class="col-6 text-center">
                        <div>{format!("BG{}", bg_number)}</div>
                        <canvas ref={self.canvas_refs[bg_number].clone()} width={THUMB_W.to_string()} height={THUMB_H.to_string()}></canvas>
                    </div>
                })}
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        self.draw();
    }
}

impl BackgroundsPanel {
    fn draw(&self) {
        let gba = self.props.gba.borrow();
        let gpu = &gba.gpu;
        let mem_map = &gba.memory_bus.mem_map;
        let mode = gpu.display_control.get_bg_mode();

        for bg_number in 0..4usize {
            let canvas = match self.canvas_refs[bg_number].cast::<HtmlCanvasElement>() {
                Some(c) => c,
                None => continue,
            };
            let ctx = match canvas.get_context("2d").ok().flatten().and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok()) {
                Some(c) => c,
                None => continue,
            };

            let mut buf = vec![0u8; THUMB_W * THUMB_H * 4];
            for pixel in buf.chunks_exact_mut(4) {
                pixel[0] = 0x18;
                pixel[1] = 0x18;
                pixel[2] = 0x1e;
                pixel[3] = 0xff;
            }

            let should_draw = gpu.display_control.should_display(bg_number as u8) && !(mode >= 3 && bg_number != 2);
            if should_draw {
                let frame = decode_background(gpu, mem_map, bg_number, mode);
                for out_y in 0..THUMB_H {
                    let src_y = out_y * GBA_HEIGHT / THUMB_H;
                    for out_x in 0..THUMB_W {
                        let src_x = out_x * GBA_WIDTH / THUMB_W;
                        let color = frame[src_y * GBA_WIDTH + src_x];
                        if color.is_transparent() {
                            continue;
                        }
                        let rgb = color.to_0rgb();
                        let idx = (out_y * THUMB_W + out_x) * 4;
                        buf[idx] = ((rgb >> 16) & 0xFF) as u8;
                        buf[idx + 1] = ((rgb >> 8) & 0xFF) as u8;
                        buf[idx + 2] = (rgb & 0xFF) as u8;
                        buf[idx + 3] = 0xFF;
                    }
                }
            }

            if let Ok(img_data) = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buf), THUMB_W as u32, THUMB_H as u32) {
                let _ = ctx.put_image_data(&img_data, 0.0, 0.0);
            }
        }
    }
}
