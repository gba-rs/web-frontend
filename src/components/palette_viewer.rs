use crate::app::App;
use yew::{html, Html};
use log::info;
use stdweb::{
    traits::*,
    unstable::TryInto,
    web::{
        document,
        html_element::CanvasElement,
        CanvasRenderingContext2d,
        event::KeyDownEvent,
        ImageData
    },
    traits::IKeyboardEvent,
};

impl App {
    pub fn convert_rgb15_rgb24(value: u16) -> u32 {
        let r = ((value & 0x1F) as u32) * 255 / 31;
        let g = (((value >> 5) & 0x1F) as u32) * 255 / 31;
        let b = (((value >> 10) & 0x1F) as u32) * 255 / 31;
        (r << 16) | (g << 8) | (b)
    }

    pub fn view_bg_palette(&self) -> Html {
        html! {
            <div>
                {for (0x500_0000u32..0x500_01FFu32).step_by(32).map(|val|{
                    html! {
                        <div style="line-height: 0">
                            {for (0..32).step_by(2).map(|offset|{
                                html!{
                                    <div class="palette_block" style={format!("background: #{:06X};", App::convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(val + offset)))}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        }
    }

    pub fn view_obj_palette(&self) -> Html {
        html! {
            <div>
                {for (0x500_0200u32..0x500_03FFu32).step_by(32).map(|val|{
                    html! {
                        <div style="line-height: 0">
                            {for (0..32).step_by(2).map(|offset|{
                                html!{
                                    <div class="palette_block" style={format!("background: #{:06X};", App::convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(val + offset)))}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        }
    }

    pub fn draw_canvas(&mut self) {
        info!("Trying to draw to the canvas");
        let canvas: CanvasElement = document()
            .query_selector("#tilemap-canvas")
            .unwrap()
            .expect("Didn't find the map canvas.")
            .try_into() // Element -> CanvasElement
            .unwrap(); // cannot be other than a canvas
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style_color("#FF0000");

        let mut tile_num: f64 = 0.;
        for i in (0x06004820..0x0600FFFF).step_by(32) {
            for y in 0..8 {    
                for x in 0..4 {
                    let index = i + (x * (y + 1));
                    let byte = self.gba.borrow().memory_bus.mem_map.read_u8(index);
                    let color_index_left = (byte & 0x0F) as u32;
                    let color_index_right = ((byte & 0xF0) >> 4) as u32;
                    let color_left =  App::convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(0x500_0000 + color_index_left));
                    let color_right =  App::convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(0x500_0000 + color_index_right));
                    log::info!("Left: #{:06X}, Right: #{:06X}", color_left, color_right);
                    context.set_fill_style_color(&format!("#{:06X}", color_left));
                    let left_x = (x as f64) + (tile_num * 8.);
                    let right_x = ((x + 1) as f64) + (tile_num * 8.);
                    context.fill_rect(left_x * 10., (y as f64) * 10. , 10., 10.);
                    context.set_fill_style_color(&format!("#{:06X}", color_right));
                    context.fill_rect(right_x * 10., (y as f64) * 10., 10., 10.);
                }   
            }
            tile_num += 1.;
        }
    }
}