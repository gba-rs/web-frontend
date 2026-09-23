use yew::{html, Component, Context, Html};
use log::info;
use crate::components::registers::{RegistersProp, RegUpdateType};

pub struct IORegisters {
    pub props: RegistersProp,
    updated_reg_hex: String,
    updated_reg_dec: String,
    update_reg_num: u8,
}

pub enum Msg {
    StartUpdate(String, RegUpdateType),
    UpdateReg(String, u8, RegUpdateType),
    FinishUpdate(RegUpdateType),
}

impl Component for IORegisters {
    type Message = Msg;
    type Properties = RegistersProp;

    fn create(ctx: &Context<Self>) -> Self {
        IORegisters {
            props: ctx.props().clone(),
            updated_reg_dec: "".to_string(),
            updated_reg_hex: "".to_string(),
            update_reg_num: 0,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartUpdate(init_string, update_type) => {
                match update_type {
                    RegUpdateType::Dec => {
                        self.updated_reg_dec = init_string;
                    },
                    RegUpdateType::Hex => {
                        self.updated_reg_hex = init_string;
                    }
                }
            },
            Msg::UpdateReg(update_string, reg_num, update_type) => {
                self.update_reg_num = reg_num;
                match update_type {
                    RegUpdateType::Hex => {
                        self.updated_reg_hex = update_string;
                    },
                    RegUpdateType::Dec => {
                        self.updated_reg_dec = update_string;
                    }
                }
            },
            Msg::FinishUpdate(update_type) => {
                match update_type {
                    RegUpdateType::Hex => {
                        self.updated_reg_hex.retain(|c| !c.is_whitespace());
                        let result = u32::from_str_radix(&self.updated_reg_hex, 16);
                        match result {
                            Ok(val) => {
//                                self.props.gba.borrow_mut().io_reg.set_register(self.update_reg_num, val)
                            },
                            Err(_) => {
                                info!("Error updating r{}: {}", self.update_reg_num, self.updated_reg_hex);
                            }
                        }
                    },
                    RegUpdateType::Dec => {
                        self.updated_reg_dec.retain(|c| !c.is_whitespace());
                        let result = u32::from_str_radix(&self.updated_reg_dec, 10);
                        match result {
                            Ok(val) => {
                                self.props.gba.borrow_mut().cpu.set_register(self.update_reg_num, val)
                            },
                            Err(_) => {
                                info!("Error updating r{}: {}", self.update_reg_num, self.updated_reg_dec);
                            }
                        }
                    }
                }
            },
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="io-reg">
                <div id="accordion">
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"LCD"}</summary>

                          {self.view_lcd()}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"Sound"}</summary>

                          {"Collapsible Group Item #2"}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"DMA"}</summary>

                          {"Collapsible Group Item #3"}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"Timer"}</summary>

                          {"Collapsible Group Item #3"}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"Serial Communication 1"}</summary>

                          {"Collapsible Group Item #3"}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"Keypad Input"}</summary>

                          {self.view_keypad_input()}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"Serial Communication 2"}</summary>

                          {"Collapsible Group Item #3"}
                      
</details>
                  <details class="io-reg-section">
<summary class="io-reg-section-header">{"Interrupt, Waitstate, and Power-Down"}</summary>

                          {self.view_interrupt_waitstate_powerdown()}
                      
</details>
                </div>
            </div>
        }
    }
}