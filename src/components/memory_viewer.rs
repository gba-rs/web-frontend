use yew::prelude::*;
use yew::{html, Component, ComponentLink, InputData, KeyboardEvent, Html, ShouldRender};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};

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
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct MemoryViewerProp {
    pub gba: Rc<RefCell<GBA>>,
    pub initialized: bool
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MemoryViewer {
            link: link,
            props: props,
            min: 0,
            max: 100,
            min_str: "".to_string(),
            max_str: "".to_string(),
            hex_string: "".to_string()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html!{
            <div class="row">
                <div class="col-3">
                    {self.view_memory_range()}
                </div>
                <div class="col-9">
                    {self.view_memory()}
                </div>
            </div>
        }
    }
}

impl MemoryViewer {
    pub fn view_memory_range(&self) -> Html {
        html! {
            <>
                <h5>{"Memory"}</h5>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="lower-addon-mem">{"Lower"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="0" oninput=self.link.callback(|e: InputData| {Msg::UpdateString(e.value, StringUpdateType::MinString)})/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="upper-addon-mem">{"Upper"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="100" oninput=self.link.callback(|e: InputData| {Msg::UpdateString(e.value, StringUpdateType::MaxString)})/>
                </div>
                <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::UpdateRange})>{"Search"}</button>
            </>
        }
    }

    pub fn view_memory(&self) -> Html {
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
                                            onclick=self.link.callback(move |_|{ Msg::StartHexEdit(format!("{:X}", byte)) })
                                            oninput=self.link.callback(move |e: InputData|{ Msg::UpdateHexString(e.value) })
                                            onkeypress=self.link.callback(move |e: KeyboardEvent|{ if e.key() == "Enter" { Msg::WriteMemory(address) } else { Msg::Nope }})/>
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