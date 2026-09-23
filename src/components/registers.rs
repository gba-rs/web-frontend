use yew::prelude::*;
use yew::{html, Component, Context, Html};
use web_sys::{InputEvent, KeyboardEvent};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};
use crate::dom_util::input_value;

pub struct Registers {
    props: RegistersProp,
    updated_reg_hex: String,
    updated_reg_dec: String,
    update_reg_num: u8,
}

#[derive(Properties, Clone)]
pub struct RegistersProp {
    pub gba: Rc<RefCell<GBA>>,
    pub hex: bool
}

impl PartialEq for RegistersProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba) && self.hex == other.hex
    }
}

pub enum RegUpdateType {
    Hex,
    Dec
}

pub enum Msg {
    StartUpdate(String, RegUpdateType),
    UpdateReg(String, u8, RegUpdateType),
    FinishUpdate(RegUpdateType),
    Nope
}

impl Component for Registers {
    type Message = Msg;
    type Properties = RegistersProp;

    fn create(ctx: &Context<Self>) -> Self {
        Registers {
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
                                self.props.gba.borrow_mut().cpu.set_register(self.update_reg_num, val)
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h4>{"Registers"}</h4>
                <table class="data-table">
                    <thead>
                        <tr>
                            <th scope="col">{"Reg"}</th>
                            <th scope="col">{"Val Hex"}</th>
                            <th scope="col">{"Val Dec"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for (0..16).map(|val|{
                            let reg_val = self.props.gba.borrow().cpu.get_register_unsafe(val);
                            let reg_num = val;
                            html! {
                                <tr>
                                    <td class="text-left">{format!("r{}", val)}</td>
                                    <td class="text-right">
                                        <input class="hex-edit hex-edit-word" type="text" value={format!("{:08X}", reg_val)}
                                        onclick={ctx.link().callback(move |_|{ Msg::StartUpdate(format!("{:08X}", reg_val), RegUpdateType::Hex) })}
                                        oninput={ctx.link().callback(move |e: InputEvent|{ Msg::UpdateReg(input_value(&e), reg_num, RegUpdateType::Hex) })}
                                        onkeypress={ctx.link().callback(|e: KeyboardEvent|{ if e.key() == "Enter" { Msg::FinishUpdate(RegUpdateType::Hex) } else { Msg::Nope } })}
                                        />
                                    </td>
                                    <td class="text-right">
                                        <input class="hex-edit hex-edit-word" type="text" value={format!("{}", reg_val)}
                                        onclick={ctx.link().callback(move |_|{ Msg::StartUpdate(format!("{}", reg_val), RegUpdateType::Dec) })}
                                        oninput={ctx.link().callback(move |e: InputEvent|{ Msg::UpdateReg(input_value(&e), reg_num, RegUpdateType::Dec) })}
                                        onkeypress={ctx.link().callback(|e: KeyboardEvent|{ if e.key() == "Enter" { Msg::FinishUpdate(RegUpdateType::Dec) } else { Msg::Nope } })}
                                        />
                                    </td>
                                </tr>
                            }                            
                        })}
                    </tbody>
                </table>
            </div>
        }
    }
}