use yew::prelude::*;
use yew::{html, Component, Context, Html, NodeRef};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;

const CELL_SIZE: usize = 20;
const CELL_INNER: usize = 18;
const COLUMNS: usize = 12;
const ROWS: usize = 11;
const WIDTH: usize = COLUMNS * CELL_SIZE;
const HEIGHT: usize = ROWS * CELL_SIZE;
const OBJ_MODE_DISABLED: u8 = 0b10;

pub struct SpritesPanel {
    canvas_ref: NodeRef,
    props: SpritesPanelProp,
}

#[derive(Properties, Clone)]
pub struct SpritesPanelProp {
    pub gba: Rc<RefCell<GBA>>,
}

impl PartialEq for SpritesPanelProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba)
    }
}

impl Component for SpritesPanel {
    type Message = ();
    type Properties = SpritesPanelProp;

    fn create(ctx: &Context<Self>) -> Self {
        SpritesPanel {
            canvas_ref: NodeRef::default(),
            props: ctx.props().clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.canvas_ref.clone()} width={WIDTH.to_string()} height={HEIGHT.to_string()}></canvas>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        self.draw();
    }
}

impl SpritesPanel {
    fn draw(&self) {
        let canvas = match self.canvas_ref.cast::<HtmlCanvasElement>() {
            Some(c) => c,
            None => return,
        };
        let ctx = match canvas.get_context("2d").ok().flatten().and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok()) {
            Some(c) => c,
            None => return,
        };

        let mut buf = vec![0u8; WIDTH * HEIGHT * 4];
        for pixel in buf.chunks_exact_mut(4) {
            pixel[0] = 0x18;
            pixel[1] = 0x18;
            pixel[2] = 0x1e;
            pixel[3] = 0xff;
        }

        let gba = self.props.gba.borrow();
        let gpu = &gba.gpu;
        let mem_map = &gba.memory_bus.mem_map;

        for i in 0..128usize {
            let col = i % COLUMNS;
            let row = i / COLUMNS;
            let cell_x = col * CELL_SIZE;
            let cell_y = row * CELL_SIZE;
            if cell_y + CELL_SIZE > HEIGHT {
                break;
            }

            if gpu.objects[i].attr0.get_obj_mode() == OBJ_MODE_DISABLED {
                continue;
            }

            let (obj_w, obj_h) = gpu.objects[i].size();
            if obj_w <= 0 || obj_h <= 0 {
                continue;
            }

            let (mut obj_x, mut obj_y) = gpu.objects[i].position();
            if obj_y >= 160 { obj_y -= 1 << 8; }
            if obj_x >= 240 { obj_x -= 1 << 9; }
            if obj_y + obj_h <= 0 || obj_y >= 160 || obj_x + obj_w <= 0 || obj_x >= 240 {
                continue;
            }

            let pixels = gpu.decode_object_pixels(i, mem_map);
            for cy in 0..CELL_INNER {
                let src_y = (cy * obj_h as usize) / CELL_INNER;
                for cx in 0..CELL_INNER {
                    let src_x = (cx * obj_w as usize) / CELL_INNER;
                    let color = pixels[src_y * obj_w as usize + src_x];
                    if color.is_transparent() {
                        continue;
                    }

                    let px = cell_x + cx;
                    let py = cell_y + cy;
                    let rgb = color.to_0rgb();
                    let idx = (py * WIDTH + px) * 4;
                    buf[idx] = ((rgb >> 16) & 0xFF) as u8;
                    buf[idx + 1] = ((rgb >> 8) & 0xFF) as u8;
                    buf[idx + 2] = (rgb & 0xFF) as u8;
                    buf[idx + 3] = 0xFF;
                }
            }
        }

        if let Ok(img_data) = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buf), WIDTH as u32, HEIGHT as u32) {
            let _ = ctx.put_image_data(&img_data, 0.0, 0.0);
        }
    }
}
