use yew::prelude::*;
use yew::{html, Component, Context, Html};
use web_sys::{InputEvent, KeyboardEvent};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};
use crate::dom_util::input_value;

pub enum StringUpdateType {
    MinString,
    MaxString
}

pub struct MemoryViewer {
    props: MemoryViewerProp,
    hex_string: String,
    pub min: u32,
    pub max: u32,
    min_str: String,
    max_str: String,
}

#[derive(Properties, Clone)]
pub struct MemoryViewerProp {
    pub gba: Rc<RefCell<GBA>>,
    pub initialized: bool
}

impl PartialEq for MemoryViewerProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba) && self.initialized == other.initialized
    }
}

pub enum Msg {
    StartHexEdit(String),
    UpdateHexString (String),
    UpdateRange,
    UpdateString(String, StringUpdateType),
    WriteMemory (u32),
    Nope
}

impl Component for MemoryViewer {
    type Message = Msg;
    type Properties = MemoryViewerProp;

    fn create(ctx: &Context<Self>) -> Self {
        MemoryViewer {
            props: ctx.props().clone(),
            min: 0,
            max: 100,
            min_str: "".to_string(),
            max_str: "".to_string(),
            hex_string: "".to_string()
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartHexEdit(init_val) => {
                self.hex_string = init_val;
                false
            },
            Msg::UpdateHexString(val) => {
                self.hex_string = val;
                false
            },
            Msg::UpdateRange => {
                let result = u32::from_str_radix(&self.max_str, 16);//self.mem_max_str.parse::<u32>();
                match result {
                    Ok(val) => {
                        self.max = val;
                    }
                    Err(_) => {}
                }

                let result = u32::from_str_radix(&self.min_str, 16);
                match result {
                    Ok(val) => {
                        self.min = val;
                    }
                    Err(_) => {}
                }
                true
            },
            Msg::UpdateString(val, range_to_update) => {
                match range_to_update {
                    StringUpdateType::MinString => {
                        self.min_str = val;
                    }
                    StringUpdateType::MaxString => {
                        self.max_str = val;
                    }
                }
                false
            },
            Msg::WriteMemory(address) => {
                self.hex_string.retain(|c| !c.is_whitespace());
                match u8::from_str_radix(&self.hex_string, 16) {
                    Ok(val) => {
                        info!("Writing value {:X}", val);
                        self.props.gba.borrow_mut().memory_bus.mem_map.write_u8(address, val);
                    },
                    Err(_) => {
                        info!("Error parsing string:{}", self.hex_string);
                    }
                }
                true
            },
            Msg::Nope => {false}
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class="debug-split">
                <div class="debug-split-side">
                    {self.view_memory_range(ctx)}
                </div>
                <div class="debug-split-main">
                    {self.view_memory(ctx)}
                </div>
            </div>
        }
    }
}

impl MemoryViewer {
    pub fn view_memory_range(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h5>{"Memory"}</h5>
                <div class="field-row">
                    <span class="field-row-label">{"Lower"}</span>
                    <input type="text" class="text-input" placeholder="0" oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateString(input_value(&e), StringUpdateType::MinString)})}/>
                </div>
                <div class="field-row">
                    <span class="field-row-label">{"Upper"}</span>
                    <input type="text" class="text-input" placeholder="100" oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateString(input_value(&e), StringUpdateType::MaxString)})}/>
                </div>
                <button class="btn" onclick={ctx.link().callback(|_|{Msg::UpdateRange})}>{"Search"}</button>
            </>
        }
    }

    pub fn view_memory(&self, ctx: &Context<Self>) -> Html {
        if self.props.initialized {
            let bytes = self.props.gba.borrow().memory_bus.mem_map.read_block(self.min, self.max - self.min);
            html! {
                <div class="code-block">
                    {for (0..bytes.len()).step_by(16).map(|val|{
                        html!{
                            <div>
                                <span class="disassembly-address">{format!("{:08X}", (self.min + val as u32))}</span>
                                {for (0..16).map(|offset|{
                                    let index = val + offset;
                                    if index < bytes.len() {
                                        let byte = bytes[val + offset];
                                        let address = self.min + val as u32 + offset as u32;
                                        html! {
                                            <input type="text" class="hex-edit hex-edit-byte" value={format!(" {:02X}", byte)}
                                            onclick={ctx.link().callback(move |_|{ Msg::StartHexEdit(format!("{:X}", byte)) })}
                                            oninput={ctx.link().callback(move |e: InputEvent|{ Msg::UpdateHexString(input_value(&e)) })}
                                            onkeypress={ctx.link().callback(move |e: KeyboardEvent|{ if e.key() == "Enter" { Msg::WriteMemory(address) } else { Msg::Nope }})}/>
                                        }
                                    } else {
                                        html! {
                                            <span>{format!(" --",)}</span>
                                        }
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div class="code-block">{"Initialize the emulator"}</div>
            }
        }
    }
}
