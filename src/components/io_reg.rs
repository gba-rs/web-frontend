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
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingOne">
                      <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
                        {"LCD"}
                      </button>
                    </div>

                    <div id="collapseOne" class="io-reg-section-body collapse" aria-labelledby="headingOne">
                      <div >
                          {self.view_lcd()}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingTwo">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                        {"Sound"}
                      </button>
                    </div>
                    <div id="collapseTwo" class="io-reg-section-body collapse" aria-labelledby="headingTwo">
                      <div >
                          {"Collapsible Group Item #2"}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingThree">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                        {"DMA"}
                      </button>
                    </div>
                    <div id="collapseThree" class="io-reg-section-body collapse" aria-labelledby="headingThree">
                      <div >
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingFour">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseFour" aria-expanded="false" aria-controls="collapseFour">
                        {"Timer"}
                      </button>
                    </div>
                    <div id="collapseFour" class="io-reg-section-body collapse" aria-labelledby="headingFour">
                      <div >
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingFive">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseFive" aria-expanded="false" aria-controls="collapseFive">
                        {"Serial Communication 1"}
                      </button>
                    </div>
                    <div id="collapseFive" class="io-reg-section-body collapse" aria-labelledby="headingFive">
                      <div >
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingSix">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseSix" aria-expanded="false" aria-controls="collapseSix">
                        {"Keypad Input"}
                      </button>
                    </div>
                    <div id="collapseSix" class="io-reg-section-body collapse" aria-labelledby="headingSix">
                      <div >
                          {self.view_keypad_input()}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingSeven">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseSeven" aria-expanded="false" aria-controls="collapseSeven">
                        {"Serial Communication 2"}
                      </button>
                    </div>
                    <div id="collapseSeven" class="io-reg-section-body collapse" aria-labelledby="headingSeven">
                      <div >
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="io-reg-section">
                    <div class="io-reg-section-header" id="headingEight">
                      <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseEight" aria-expanded="false" aria-controls="collapseEight">
                        {"Interrupt, Waitstate, and Power-Down"}
                      </button>
                    </div>
                    <div id="collapseEight" class="io-reg-section-body collapse" aria-labelledby="headingEight">
                      <div >
                          {self.view_interrupt_waitstate_powerdown()}
                      </div>
                    </div>
                  </div>
                </div>
            </div>
        }
    }
}