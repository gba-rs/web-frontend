use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{html, Component, ComponentLink, InputData, Html, ShouldRender};

use gba_emulator::{
    gba::GBA,
    gamepak::GamePack,
    cpu::{
        cpu::InstructionSet,
        cpu::ARM_PC,
        cpu::THUMB_PC
    },
};

use std::rc::Rc;
use std::cell::RefCell;
use log::{info, error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::{
    registers::Registers,
    navbar::NavBar,
    cpsr::Cpsr,
    status::Status,
    memory_viewer::MemoryViewer,
    io_reg::IORegisters,
    disassembler::DisassemblyElement,
    canvas::*,
};

use crate::logging;

pub const START_PC: u32 = 0;
pub static mut GO: bool = false;

pub struct App {
    pub reader: ReaderService,
    pub tasks: Vec<ReaderTask>,
    pub rom_name: String,
    pub bios_name: String,
    pub save_name: String,
    pub save: Vec<u8>,
    pub disassembly: Vec<DisassemblyElement>,
    pub gba: Rc<RefCell<GBA>>,
    pub game_pack: GamePack,
    pub link: ComponentLink<App>,
    pub hex: bool,
    pub follow_pc: bool,
    pub initialized: bool,
    pub disassembled: bool,
    pub dis_min: u32,
    pub dis_max: u32,
    pub mem_min: u32,
    pub mem_max: u32,
    pub dis_min_str: String,
    pub dis_max_str: String,
    pub mem_min_str: String,
    pub mem_max_str: String,
    pub run_addr_str: String,
    pub active_menu: ActiveMenu,
}

pub enum RangeUpdate {
    DisassemblyMin,
    DisassemblyMax,
}

pub enum FileLoadType {
    Rom,
    Bios,
    Save
}

pub enum Msg {
    LoadedRom(FileData),
    LoadedBios(FileData),
    LoadedSave(FileData),
    Init,
    Step(u8),
    Run(u32),
    Files(Vec<File>, FileLoadType),
    ToggleFollow,
    ToggleMenu(ActiveMenu),
    UpdateRange(RangeUpdate),
    UpdateInputString(String, RangeUpdate),
    UpdateRunString(String),
    StartRun,
    Frame,
    Go,
    Stop,
    ToggleLog
}

#[derive(PartialEq)]
pub enum ActiveMenu {
    Registers,
    IO,
    Graphics,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        match logging::init_logger() {
            Ok(_) => {
                info!("Logger initialized succesfully");
            }
            Err(_) => {
                info!("Logger failed to initialize");
            }
        }

        info!("Created Application");
        App {
            reader: ReaderService::new(),
            link,
            rom_name: "Choose File".to_string(),
            bios_name: "Choose File".to_string(),
            save_name: "Choose File (Optional)".to_string(),
            save: vec![],
            disassembly: vec![],
            gba: Rc::new(RefCell::new(GBA::default())),
            game_pack: GamePack::default(),
            tasks: vec![],
            hex: false,
            follow_pc: true,
            initialized: false,
            disassembled: false,
            dis_min: 0,
            dis_max: 100,
            mem_min: 0,
            mem_max: 100,
            dis_min_str: "".to_string(),
            dis_max_str: "".to_string(),
            mem_min_str: "".to_string(),
            mem_max_str: "".to_string(),
            run_addr_str: "".to_string(),
            active_menu: ActiveMenu::Registers,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadedRom(file) => {
                self.game_pack.rom = file.content;
                self.rom_name = file.name;
                self.disassembled = false;
                self.initialized = false;
                true
            }
            Msg::LoadedBios(file) => {
                self.game_pack.bios = file.content;
                self.bios_name = file.name;
                self.initialized = false;
                true
            }
            Msg::LoadedSave(file) => {
                self.save = file.content;
                self.save_name = file.name;
                true
            }
            Msg::Init => {
                self.game_pack.backup_type = GamePack::detect_backup_type(&self.game_pack.rom);
                self.gba = Rc::new(RefCell::new(GBA::new(START_PC, &self.game_pack)));
                if self.save.len() != 0 {
                    self.gba.borrow_mut().load_save_file(&self.save);
                }
                self.initialized = true;
                clear_canvas();
                info!("Created new Emulator");

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

                self.gba.borrow_mut().key_status.set_register(0xFFFF);

                // need 2 because we move the clone into the closure
                let key_down_clone = self.gba.clone();
                let key_up_clone = self.gba.clone();

                log::info!("Setup key bindings");
                let key_down = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                    unsafe {
                        if GO {
                            match event.key_code() {
                                web_sys::KeyEvent::DOM_VK_W => key_down_clone.borrow_mut().key_status.set_dpad_up(0),
                                web_sys::KeyEvent::DOM_VK_S => key_down_clone.borrow_mut().key_status.set_dpad_down(0),
                                web_sys::KeyEvent::DOM_VK_A => key_down_clone.borrow_mut().key_status.set_dpad_left(0),
                                web_sys::KeyEvent::DOM_VK_D => key_down_clone.borrow_mut().key_status.set_dpad_right(0),
                                web_sys::KeyEvent::DOM_VK_H => key_down_clone.borrow_mut().key_status.set_button_a(0),
                                web_sys::KeyEvent::DOM_VK_J => key_down_clone.borrow_mut().key_status.set_button_b(0),
                                web_sys::KeyEvent::DOM_VK_E => key_down_clone.borrow_mut().key_status.set_button_r(0),
                                web_sys::KeyEvent::DOM_VK_Q => key_down_clone.borrow_mut().key_status.set_button_l(0),
                                web_sys::KeyEvent::DOM_VK_BACK_SPACE => key_down_clone.borrow_mut().key_status.set_button_select(0),
                                web_sys::KeyEvent::DOM_VK_RETURN => key_down_clone.borrow_mut().key_status.set_button_start(0),
                                _ => {}
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let key_up = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                    unsafe {
                        if GO {
                            match event.key_code() {
                                web_sys::KeyEvent::DOM_VK_W => key_up_clone.borrow_mut().key_status.set_dpad_up(1),
                                web_sys::KeyEvent::DOM_VK_S => key_up_clone.borrow_mut().key_status.set_dpad_down(1),
                                web_sys::KeyEvent::DOM_VK_A => key_up_clone.borrow_mut().key_status.set_dpad_left(1),
                                web_sys::KeyEvent::DOM_VK_D => key_up_clone.borrow_mut().key_status.set_dpad_right(1),
                                web_sys::KeyEvent::DOM_VK_H => key_up_clone.borrow_mut().key_status.set_button_a(1),
                                web_sys::KeyEvent::DOM_VK_J => key_up_clone.borrow_mut().key_status.set_button_b(1),
                                web_sys::KeyEvent::DOM_VK_E => key_up_clone.borrow_mut().key_status.set_button_r(1),
                                web_sys::KeyEvent::DOM_VK_Q => key_up_clone.borrow_mut().key_status.set_button_l(1),
                                web_sys::KeyEvent::DOM_VK_BACK_SPACE => key_up_clone.borrow_mut().key_status.set_button_select(1),
                                web_sys::KeyEvent::DOM_VK_RETURN => key_up_clone.borrow_mut().key_status.set_button_start(1),
                                _ => {}
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
        
                window().set_onkeydown(Some(key_down.as_ref().unchecked_ref()));
                window().set_onkeyup(Some(key_up.as_ref().unchecked_ref()));

                key_down.forget();
                key_up.forget();

                true
            }
            Msg::Step(step_count) => {
                for _ in 0..step_count {
                    self.gba.as_ref().borrow_mut().single_step();
                }

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

                true
            },
            Msg::Frame => {
                self.gba.borrow_mut().frame();
                show_canvas(convert_frame_to_u8(&self.gba.borrow().gpu.frame_buffer));

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

                true
            },
            Msg::UpdateRunString(value) => {
                self.run_addr_str = value;
                false
            }
            Msg::StartRun => {
                let result = u32::from_str_radix(&self.run_addr_str, 16);//self.mem_max_str.parse::<u32>();
                match result {
                    Ok(val) => {
                        self.link.send_message(Msg::Run(val));
                    }
                    Err(_) => {
                        error!("Error parsing run address");
                    }
                }

                false
            }
            Msg::ToggleLog => {
                unsafe {
                    crate::logging::LOGGER.should_log = !crate::logging::LOGGER.should_log;
                }
                false
            }
            Msg::Run(address) => {
                self.gba.borrow_mut().single_step();
                let mut current_pc = if self.gba.borrow().cpu.get_instruction_set() == InstructionSet::Arm { self.gba.borrow().cpu.get_register(ARM_PC) } else { self.gba.borrow().cpu.get_register(THUMB_PC) };

                while current_pc != address {
                    self.gba.borrow_mut().single_step();
                    current_pc = self.gba.borrow_mut().cpu.get_register_unsafe(15);
                }

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

                true
            }
            Msg::ToggleFollow => {
                self.follow_pc = !self.follow_pc;
                true
            }
            Msg::UpdateRange(range_to_update) => {
                match range_to_update {
                    RangeUpdate::DisassemblyMin | RangeUpdate::DisassemblyMax => {
                        let result = u32::from_str_radix(&self.dis_max_str, 16);
                        match result {
                            Ok(val) => {
                                self.dis_max = val;
                            }
                            Err(_) => {}
                        }

                        let result = u32::from_str_radix(&self.dis_min_str, 16);
                        match result {
                            Ok(val) => {
                                self.dis_min = val;
                            }
                            Err(_) => {}
                        }

                        if !self.follow_pc {
                            self.disassembly.clear();
                            let total_bytes = (self.dis_max as i64 - self.dis_min as i64) as u32;
                            self.disassemble(self.dis_min, total_bytes);
                        }
                    }
                }
                true
            }
            Msg::UpdateInputString(val, range_to_update) => {
                match range_to_update {
                    RangeUpdate::DisassemblyMin => {
                        self.dis_min_str = val;
                    }
                    RangeUpdate::DisassemblyMax => {
                        self.dis_max_str = val;
                    }
                }
                false
            }
            Msg::Files(files, file_type) => {
                for file in files.into_iter() {
                    let task_result = {
                        match file_type {
                            FileLoadType::Rom => {
                                let callback = self.link.callback(Msg::LoadedRom);
                                self.reader.read_file(file, callback)
                            },
                            FileLoadType::Bios => {
                                let callback = self.link.callback(Msg::LoadedBios);
                                self.reader.read_file(file, callback)
                            },
                            FileLoadType::Save => {
                                let callback = self.link.callback(Msg::LoadedSave);
                                self.reader.read_file(file, callback)
                            }
                        }
                    };
                    self.tasks.push(task_result.unwrap());
                }
                false
            }
            Msg::ToggleMenu(menu_item) => {
                self.active_menu = menu_item;
                true
            },
            Msg::Go => {
                unsafe {
                    GO = true;
                }

                let gba_clone = self.gba.clone();
                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    unsafe {    
                        if !GO {
                            let _ = f.borrow_mut().take();
                            return;
                        }
                    }
                    gba_clone.borrow_mut().frame();
                    // gba_clone.borrow_mut().key_status.set_register(0xFFFF);
                    show_canvas(convert_frame_to_u8(&gba_clone.borrow().gpu.frame_buffer));

                    request_animation_frame(f.borrow().as_ref().unwrap());
                }) as Box<dyn FnMut()>));

                request_animation_frame(g.borrow().as_ref().unwrap());
                true
            },
            Msg::Stop => {
                unsafe {
                    GO = false;
                }
                if self.follow_pc {
                    self.follow_pc_disassemble();
                }
                true
            },
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <NavBar/>
                <div class="container-fluid">
                    <div class="row">
                        {self.view_control()}
                    </div>
                    <div class="row">
                         <div class="col-xs-12 col-lg-6 col-xl-6">
                             <ul class="nav nav-tabs">
                               <li class="nav-item"><a class={format!("nav-link {}",self.is_menu_tab_active(ActiveMenu::Registers))} href="#" onclick=self.link.callback(|_|{Msg::ToggleMenu(ActiveMenu::Registers)})>{"Registers/Status"}</a></li>
                               <li class="nav-item"><a class={format!("nav-link {}",self.is_menu_tab_active(ActiveMenu::IO))} href="#" onclick=self.link.callback(|_|{Msg::ToggleMenu(ActiveMenu::IO)})>{"IO Registers"}</a></li>
                               <li class="nav-item"><a class={format!("nav-link {}",self.is_menu_tab_active(ActiveMenu::Graphics))} href="#" onclick=self.link.callback(|_|{Msg::ToggleMenu(ActiveMenu::Graphics)})>{"Graphics"}</a></li>
                             </ul>
                             <div class={format!("row {}", self.is_menu_body_active(ActiveMenu::Registers))}>
                                 <div class="col-xs-12 col-lg-6 col-xl-6">
                                    <Status gba={self.gba.clone()}/>
                                    <Cpsr gba={self.gba.clone()}/>
                                </div>

                                <div class="col-xs-12 col-lg-6 col-xl-6">
                                    <Registers hex={self.hex} gba={self.gba.clone()}/>
                                </div>
                             </div>
                             <div class={format!("row {}", self.is_menu_body_active(ActiveMenu::IO))}>
                                <div class="col-xs-12 col-lg-12 col-xl-12">
                                    <IORegisters hex={self.hex} gba={self.gba.clone()}/>
                                </div>
                             </div>
                             <div class={format!("row {}", self.is_menu_body_active(ActiveMenu::Graphics))}>
                                <div class="col-xs-12 col-lg-12 col-xl-12 text-center">
                                        {self.view_canvas()}
                                </div>
                             </div>
                             <div class={format!("row {}", self.is_menu_body_active(ActiveMenu::Graphics))}>
                                    <div class="col-xs-1 col-lg-1 col-xl-1"></div>
                                    <div class="col-xs-5 col-lg-5 col-xl-5 text-center">
                                        <h5>{"Background Palette"}</h5>
                                        {self.view_bg_palette()}
                                    </div>
                                    <div class="col-xs-5 col-lg-5 col-xl-5 text-center">
                                        <h5>{"Object Palette"}</h5>
                                        {self.view_obj_palette()}
                                    </div>
                                    <div class="col-xs-1 col-lg-1 col-xl-1"></div>
                            </div>
                         </div>

                        <div class="col-xs-12 col-xl-6">
                            <div class="row">
                                <div class="col-3">
                                    {self.view_range_dis()}
                                </div>
                                <div class="col-9">
                                    {self.view_disassembly()}
                                </div>
                            </div>
                            <MemoryViewer gba={self.gba.clone()} initialized={self.initialized}/>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

impl App {
    pub fn view_range_dis(&self) -> Html {
        html! {
            <>
                <h5>{"Disassembly"}</h5>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="lower-addon-dis">{"Lower"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="0" oninput=self.link.callback(|e: InputData| {Msg::UpdateInputString(e.value, RangeUpdate::DisassemblyMin)})/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="upper-addon-dis">{"Upper"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="100" oninput=self.link.callback(|e: InputData| {Msg::UpdateInputString(e.value, RangeUpdate::DisassemblyMax)})/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="follow-addon">{"Follow PC"}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={self.follow_pc} onclick=self.link.callback(|_|{Msg::ToggleFollow})/>
                        </div>                                
                    </div>
                </div>
                <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::UpdateRange(RangeUpdate::DisassemblyMax)})>{"Search"}</button>
            </>
        }
    }

    pub fn is_menu_tab_active(&self, menu_item: ActiveMenu) -> String {
        if menu_item == self.active_menu {
            return format!("active");
        }
        return format!("");
    }

    pub fn is_menu_body_active(&self, menu_item: ActiveMenu) -> String {
        if menu_item != self.active_menu {
            return format!("d-none");
        }
        return format!("");
    }
}
