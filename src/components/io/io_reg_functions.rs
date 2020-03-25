use yew::{html, Html};
use crate::components::io_reg::{IORegisters};
use crate::components::io::lcd::LCD;

impl IORegisters {
    pub fn view_lcd(&self) -> Html {
        html! {
            <div id="lcd-accordion">
                {self.view_display_control()}
                {self.view_display_status()}
                {self.view_green_swap()}
                {self.view_bg()}
            </div>
        }
    }

}
