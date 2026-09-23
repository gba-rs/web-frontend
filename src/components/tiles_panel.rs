use yew::prelude::*;
use yew::{html, Component, Context, Html, NodeRef};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use gba_emulator::gba::GBA;
use gba_emulator::memory::memory_map::{PALETTE_RAM_START, PALETTE_RAM_SIZE};
use gba_emulator::memory::lcd_io_registers::PixelFormat;
use gba_emulator::gpu::rgb15::Rgb15;
use std::rc::Rc;
use std::cell::RefCell;

const TILE_SIZE: usize = 8;
const COLUMNS: usize = 24;
const ROWS: usize = 16;
const WIDTH: usize = COLUMNS * TILE_SIZE;
const HEIGHT: usize = ROWS * TILE_SIZE;

pub struct TilesPanel {
    canvas_ref: NodeRef,
    props: TilesPanelProp,
}

#[derive(Properties, Clone)]
pub struct TilesPanelProp {
    pub gba: Rc<RefCell<GBA>>,
}

impl PartialEq for TilesPanelProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba)
    }
}

impl Component for TilesPanel {
    type Message = ();
    type Properties = TilesPanelProp;

    fn create(ctx: &Context<Self>) -> Self {
        TilesPanel {
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

impl TilesPanel {
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

        let bg_number = (0..4).find(|&i| gpu.display_control.should_display(i)).unwrap_or(0);
        let control = &gpu.backgrounds[bg_number as usize].control;
        let tileset_location = control.get_tileset_location();
        let pixel_format = control.get_pixel_format();
        let bytes_per_tile = control.get_tilesize();

        let vram_char_block_size = 0x4000u32;
        let tile_count = (vram_char_block_size / bytes_per_tile).min((COLUMNS * ROWS) as u32);

        for tile_index in 0..tile_count {
            let col = (tile_index as usize) % COLUMNS;
            let row = (tile_index as usize) / COLUMNS;
            if row >= ROWS {
                break;
            }
            let cell_x = col * TILE_SIZE;
            let cell_y = row * TILE_SIZE;

            let tile_address = tileset_location + tile_index * bytes_per_tile;
            for py in 0..TILE_SIZE {
                for px in 0..TILE_SIZE {
                    let pixel_index = match pixel_format {
                        PixelFormat::EightBit => {
                            let addr = tile_address + (8 * py as u32 + px as u32);
                            mem_map.memory[addr as usize].get()
                        }
                        PixelFormat::FourBit => {
                            let addr = tile_address + (4 * py as u32 + (px as u32 / 2));
                            let value = mem_map.memory[addr as usize].get();
                            if px & 1 != 0 { value >> 4 } else { value & 0xf }
                        }
                    } as u32;

                    let color = if pixel_index == 0 {
                        Rgb15::new(0x8000)
                    } else {
                        let palette_ram_index = 2 * pixel_index;
                        let raw_addr = palette_ram_index + 0x500_0000u32;
                        let masked_addr = (raw_addr & PALETTE_RAM_SIZE) + PALETTE_RAM_START;
                        let idx = masked_addr as usize;
                        let value = u16::from_le_bytes([mem_map.memory[idx].get(), mem_map.memory[idx + 1].get()]);
                        Rgb15::new(value)
                    };

                    if color.is_transparent() {
                        continue;
                    }

                    let out_x = cell_x + px;
                    let out_y = cell_y + py;
                    if out_x < WIDTH && out_y < HEIGHT {
                        let rgb = color.to_0rgb();
                        let idx = (out_y * WIDTH + out_x) * 4;
                        buf[idx] = ((rgb >> 16) & 0xFF) as u8;
                        buf[idx + 1] = ((rgb >> 8) & 0xFF) as u8;
                        buf[idx + 2] = (rgb & 0xFF) as u8;
                        buf[idx + 3] = 0xFF;
                    }
                }
            }
        }

        if let Ok(img_data) = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buf), WIDTH as u32, HEIGHT as u32) {
            let _ = ctx.put_image_data(&img_data, 0.0, 0.0);
        }
    }
}
