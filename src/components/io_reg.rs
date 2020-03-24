use yew::prelude::*;
use yew::{html, Component, ComponentLink, InputData, KeyPressEvent, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::InstructionSet;
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};
use crate::components::registers::{RegistersProp, RegUpdateType};

pub struct IORegisters {
    pub props: RegistersProp,
    updated_reg_hex: String,
    updated_reg_dec: String,
    update_reg_num: u8,
    link: ComponentLink<Self>,
    tab_states: TabStates
}

pub enum Msg {
    StartUpdate(String, RegUpdateType),
    UpdateReg(String, u8, RegUpdateType),
    FinishUpdate(RegUpdateType),
    Nope
}

#[derive(PartialEq, Clone, Copy)]
pub enum TabState {
    Closed,
    Open,
}

pub struct TabStates {
    lcd: TabState,
    sound: TabState,
    dma: TabState,
    timer: TabState,
    sc1: TabState,
    keypad: TabState,
    sc2: TabState,
    int_ws_pd: TabState,
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
            tab_states: TabStates {
                lcd: TabState::Closed,
                sound: TabState::Closed,
                dma: TabState::Closed,
                timer: TabState::Closed,
                sc1: TabState::Closed,
                keypad: TabState::Closed,
                sc2: TabState::Closed,
                int_ws_pd: TabState::Closed
            }
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
//                <table class="table register-table">
//                    <thead>
//                        <tr>
//                            <th scope="col">{"Reg"}</th>
//                            <th scope="col">{"Val Hex"}</th>
//                            <th scope="col">{"Val Dec"}</th>
//                        </tr>
//                    </thead>
//                    <tbody>
//                        {for (0..if self.props.gba.borrow().cpu.current_instruction_set == InstructionSet::Arm { 16 } else { 11 }).map(|val|{
//                            let reg_val = self.props.gba.borrow().io_reg.get_register(val);
//                            let reg_num = val;
//                            html! {
//                                <tr>
//                                    <td class="text-left">{format!("r{}", val)}</td>
//                                    <td class="text-right">
//                                        <input class="hex-edit hex-edit-word" type="text" value={format!("{:08X}", reg_val)}
//                                        onclick=self.link.callback(move |_|{ Msg::StartUpdate(format!("{:08X}", reg_val), RegUpdateType::Hex) })
//                                        oninput=self.link.callback(move |e: InputData|{ Msg::UpdateReg(e.value, reg_num, RegUpdateType::Hex) })
//                                        onkeypress=self.link.callback(|e: KeyPressEvent|{ if e.key() == "Enter" { Msg::FinishUpdate(RegUpdateType::Hex) } else { Msg::Nope } })
//                                        />
//                                    </td>
//                                    <td class="text-right">
//                                        <input class="hex-edit hex-edit-word" type="text" value={format!("{}", reg_val)}
//                                        onclick=self.link.callback(move |_|{ Msg::StartUpdate(format!("{}", reg_val), RegUpdateType::Dec) })
//                                        oninput=self.link.callback(move |e: InputData|{ Msg::UpdateReg(e.value, reg_num, RegUpdateType::Dec) })
//                                        onkeypress=self.link.callback(|e: KeyPressEvent|{ if e.key() == "Enter" { Msg::FinishUpdate(RegUpdateType::Dec) } else { Msg::Nope } })
//                                        />
//                                    </td>
//                                </tr>
//                            }
//                        })}
//                    </tbody>
//                </table>
            <div id="accordion">
              <div class="card">
                <div class="card-header p-0" id="headingOne">
                  <h5 class="mb-0">
                    <button class="btn btn-link" data-toggle="collapse" data-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
                      {"LCD"}
                    </button>
                  </h5>
                </div>

                <div id="collapseOne" class="collapse show" aria-labelledby="headingOne">
                  <div class="card-body">
                      {self.view_lcd()}
                  </div>
                </div>
              </div>
              <div class="card">
                <div class="card-header p-0" id="headingTwo">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
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
              <div class="card">
                <div class="card-header p-0" id="headingThree">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
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
              <div class="card">
                <div class="card-header p-0" id="headingFour">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseFour" aria-expanded="false" aria-controls="collapseFour">
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
              <div class="card">
                <div class="card-header p-0" id="headingFive">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseFive" aria-expanded="false" aria-controls="collapseFive">
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
              <div class="card">
                <div class="card-header p-0" id="headingSix">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseSix" aria-expanded="false" aria-controls="collapseSix">
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
              <div class="card">
                <div class="card-header p-0" id="headingSeven">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseSeven" aria-expanded="false" aria-controls="collapseSeven">
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
              <div class="card">
                <div class="card-header p-0" id="headingEight">
                  <h5 class="mb-0">
                    <button class="btn btn-link collapsed" data-toggle="collapse" data-target="#collapseEight" aria-expanded="false" aria-controls="collapseEight">
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