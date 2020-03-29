use yew::prelude::*;
use yew::{html, Component, ComponentLink, InputData, KeyboardEvent, Html, ShouldRender};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;
use log::{info};

pub struct MemoryViewer {
    props: MemoryViewerProp,
    hex_string: String,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone)]
pub struct MemoryViewerProp {
    pub gba: Rc<RefCell<GBA>>,
    pub min: u32,
    pub max: u32,
    pub initialized: bool
}

pub enum Msg {
    StartHexEdit(String),
    UpdateHexString (String),
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
        if self.props.initialized {
            let bytes = self.props.gba.borrow().memory_bus.mem_map.read_block(self.props.min, self.props.max - self.props.min);
            html! {
                <div class="code-block">
                    {for (0..bytes.len()).step_by(16).map(|val|{
                        html!{
                            <div>
                                <span class="disassembly-address">{format!("{:08X}", (self.props.min + val as u32))}</span>
                                {for (0..16).map(|offset|{
                                    let index = val + offset;
                                    if index < bytes.len() {
                                        let byte = bytes[val + offset];
                                        let address = self.props.min + val as u32 + offset as u32;
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