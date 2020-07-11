use yew::{html, Html};
use crate::components::io_reg::{IORegisters};

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
                {self.view_mosaic_size()}
                {self.view_color_spec_effect_selection()}
                {self.view_alpha_blending()}
                {self.view_brightness()}
            </div>
        }
    }

    pub fn view_keypad_input(&self) -> Html {
        html! {
            <div id="keypad-accordion">
                {self.view_key_status()}
                {self.view_key_interrupt_control()}
            </div>
        }
    }

    pub fn view_interrupt_waitstate_powerdown(&self) -> Html {
        html! {
            <div id="iwspd-accordion">
                {self.view_interrupt_master_enable_register()}
                {self.view_interrupt_enable_register()}
                {self.view_interrupt_request_flags()}
                {self.view_wait_state()}
            </div>
        }
    }

}
