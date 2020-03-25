use yew::prelude::*;
use yew::{html, Component, ComponentLink, InputData, KeyPressEvent, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};
use crate::components::registers::{RegistersProp, RegUpdateType};
use crate::components::io::lcd::LCD;

pub struct IORegisters {
    pub props: RegistersProp,
    updated_reg_hex: String,
    updated_reg_dec: String,
    update_reg_num: u8,
    link: ComponentLink<Self>,
}

pub enum Msg {
    StartUpdate(String, RegUpdateType),
    UpdateReg(String, u8, RegUpdateType),
    FinishUpdate(RegUpdateType),
    Nope
}

impl Component for IORegisters {
    type Message = Msg;
    type Properties = RegistersProp;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        IORegisters {
            props: props,
            updated_reg_dec: "".to_string(),
            updated_reg_hex: "".to_string(),
            update_reg_num: 0,
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            Msg::Nope => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div id="accordion">
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingOne">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
                          {"LCD"}
                        </button>
                      </h5>
                    </div>

                    <div id="collapseOne" class="collapse" aria-labelledby="headingOne">
                      <div class="card-body">
                          {self.view_lcd()}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingTwo">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                          {"Sound"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseTwo" class="collapse" aria-labelledby="headingTwo">
                      <div class="card-body">
                          {"Collapsible Group Item #2"}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingThree">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                          {"DMA"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseThree" class="collapse" aria-labelledby="headingThree">
                      <div class="card-body">
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingFour">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseFour" aria-expanded="false" aria-controls="collapseFour">
                          {"Timer"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseFour" class="collapse" aria-labelledby="headingFour">
                      <div class="card-body">
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingFive">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseFive" aria-expanded="false" aria-controls="collapseFive">
                          {"Serial Communication 1"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseFive" class="collapse" aria-labelledby="headingFive">
                      <div class="card-body">
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingSix">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseSix" aria-expanded="false" aria-controls="collapseSix">
                          {"Keypad Input"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseSix" class="collapse" aria-labelledby="headingSix">
                      <div class="card-body">
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingSeven">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseSeven" aria-expanded="false" aria-controls="collapseSeven">
                          {"Serial Communication 2"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseSeven" class="collapse" aria-labelledby="headingSeven">
                      <div class="card-body">
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                  <div class="card border-0">
                    <div class="card-header p-0 bg-transparent" id="headingEight">
                      <h5 class="mb-0">
                        <button class="btn btn-link text-dark collapsed" data-toggle="collapse" data-target="#collapseEight" aria-expanded="false" aria-controls="collapseEight">
                          {"Interrupt, Waitstate, and Power-Down"}
                        </button>
                      </h5>
                    </div>
                    <div id="collapseEight" class="collapse" aria-labelledby="headingEight">
                      <div class="card-body">
                          {"Collapsible Group Item #3"}
                      </div>
                    </div>
                  </div>
                </div>
            </div>
        }
    }
}