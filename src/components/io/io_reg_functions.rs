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
                {self.view_bg(0)}
                {self.view_bg(1)}
                {self.view_bg(2)}
                {self.view_bg(3)}
                {self.view_bg_affine_component(0)}
                {self.view_bg_affine_component(1)}
                {self.view_window(0)}
                {self.view_window(1)}
                {self.view_control_window_inside()}
                {self.view_control_window_outside()}
            </div>
        }
    }

}
