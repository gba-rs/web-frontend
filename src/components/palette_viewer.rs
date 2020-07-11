use crate::app::App;
use yew::{html, Html};

fn convert_rgb15_rgb24(value: u16) -> u32 {
    let r = ((value & 0x1F) as u32) * 255 / 31;
    let g = (((value >> 5) & 0x1F) as u32) * 255 / 31;
    let b = (((value >> 10) & 0x1F) as u32) * 255 / 31;
    (r << 16) | (g << 8) | (b)
}

impl App {
    pub fn view_bg_palette(&self) -> Html {
        html! {
            <div>
                {for (0x500_0000u32..0x500_01FFu32).step_by(32).map(|val|{
                    html! {
                        <div style="line-height: 0">
                            {for (0..32).step_by(2).map(|offset|{
                                html!{
                                    <div class="palette_block" style={format!("background: #{:06X};", convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(val + offset)))}></div>
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
                                    <div class="palette_block" style={format!("background: #{:06X};", convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(val + offset)))}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        }
    }
}