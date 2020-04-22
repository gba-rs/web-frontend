use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::{html, Component, ComponentLink, InputData, Html, ShouldRender};
use gba_emulator::gba::GBA;
use gba_emulator::gamepak::GamePack;
use gba_emulator::cpu::{cpu::InstructionSet, cpu::ARM_PC, cpu::THUMB_PC};
use gba_emulator::gpu::{gpu::DISPLAY_HEIGHT, gpu::DISPLAY_WIDTH};
use std::rc::Rc;
use std::cell::RefCell;
use log::{info, error};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, Clamped};
use web_sys::{ImageData, HtmlImageElement};


use crate::components::{
    registers::Registers,
    navbar::NavBar,
    cpsr::Cpsr,
    status::Status,
    memory_viewer::MemoryViewer,
    io_reg::IORegisters
};

use crate::logging;

pub const START_PC: u32 = 0;
pub const FOLLOW_MIN: i64 = -10;
pub const FOLLOW_MAX: i64 = 10;

struct DisassemblyElement {
    address: u32,
    instruction_hex: u32,
    instruction_asm: String,
}


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// export a Rust function that uses the imported JS function
#[wasm_bindgen]
pub fn show_canvas(mut pixels: Vec<u8>) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("gba-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(DISPLAY_WIDTH);
    canvas.set_height(DISPLAY_HEIGHT);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut img = document.get_element_by_id("gba-img").unwrap().dyn_into::<web_sys::HtmlImageElement>().unwrap();
    let mut img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels), canvas.width(), canvas.height()).unwrap();
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.put_image_data(&img_data, 0.0, 0.0 );
    img.set_src(canvas.to_data_url().unwrap().as_str());

    let scale = 3;
    canvas.set_width(DISPLAY_WIDTH * scale);
    canvas.set_height(DISPLAY_HEIGHT * scale);
    context.scale(scale as f64, scale as f64);
    context.draw_image_with_html_image_element(&img, 0.0, 0.0);
}

//#[wasm_bindgen]
//pub fn go() {
//
//}



pub struct App {
    reader: ReaderService,
    tasks: Vec<ReaderTask>,
    rom: Vec<u8>,
    bios: Vec<u8>,
    rom_name: String,
    bios_name: String,
    disassembly: Vec<DisassemblyElement>,
    gba: Rc<RefCell<GBA>>,
    game_pack: GamePack,
    link: ComponentLink<App>,
    hex: bool,
    follow_pc: bool,
    initialized: bool,
    disassembled: bool,
    dis_min: u32,
    dis_max: u32,
    mem_min: u32,
    mem_max: u32,
    dis_min_str: String,
    dis_max_str: String,
    mem_min_str: String,
    mem_max_str: String,
    run_addr_str: String,
    active_menu: ActiveMenu,
}

pub enum RangeUpdate {
    MemoryViewerMin,
    MemoryViewerMax,
    DisassemblyMin,
    DisassemblyMax,
}

pub enum Msg {
    LoadedRom(FileData),
    LoadedBios(FileData),
    Init,
    Step(u8),
    Run(u32),
    Files(Vec<File>, bool),
    ToggleFollow,
    ToggleMenu(ActiveMenu),
    UpdateRange(RangeUpdate),
    UpdateInputString(String, RangeUpdate),
    UpdateRunString(String),
    StartRun,
    Frame,
    Go,
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
            bios: vec![],
            rom: vec![],
            rom_name: "Choose File".to_string(),
            bios_name: "Choose File".to_string(),
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
            Msg::Init => {
                self.game_pack.backup_type = GamePack::detect_backup_type(&self.game_pack.rom);
                self.gba = Rc::new(RefCell::new(GBA::new(START_PC, &self.game_pack)));
                self.initialized = true;
                info!("Created new Emulator");

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

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
                    RangeUpdate::MemoryViewerMin | RangeUpdate::MemoryViewerMax => {
                        let result = u32::from_str_radix(&self.mem_max_str, 16);//self.mem_max_str.parse::<u32>();
                        match result {
                            Ok(val) => {
                                self.mem_max = val;
                            }
                            Err(_) => {}
                        }

                        let result = u32::from_str_radix(&self.mem_min_str, 16);
                        match result {
                            Ok(val) => {
                                self.mem_min = val;
                            }
                            Err(_) => {}
                        }
                    }
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
                    RangeUpdate::MemoryViewerMin => {
                        self.mem_min_str = val;
                    }
                    RangeUpdate::MemoryViewerMax => {
                        self.mem_max_str = val;
                    }
                    RangeUpdate::DisassemblyMin => {
                        self.dis_min_str = val;
                    }
                    RangeUpdate::DisassemblyMax => {
                        self.dis_max_str = val;
                    }
                }
                false
            }
            Msg::Files(files, rom) => {
                for file in files.into_iter() {
                    let task_result = {
                        if rom {
                            let callback = self.link.callback(Msg::LoadedRom);
                            self.reader.read_file(file, callback)
                        } else {
                            let callback = self.link.callback(Msg::LoadedBios);
                            self.reader.read_file(file, callback)
                        }
                    };
                    self.tasks.push(task_result.unwrap());
                }
                false
            }
            Msg::ToggleMenu(menu_item) => {
                self.active_menu = menu_item;
                true
            }
            Msg::Frame => {
                self.gba.borrow_mut().frame();

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }
                show_canvas(convert_frame_to_u8(&self.gba.borrow().gpu.frame_buffer));
                true
            },
            Msg::Go => {

                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                let mut i = 0;
                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    if i > 300 {
                        info!("{}", "ALL DONE");

                        // Drop our handle to this closure so that it will get cleaned
                        // up once we return.
                        let _ = f.borrow_mut().take();
                        return;
                    }

                    i += 1;
                    info!("RUNNING {}", i);
                    self.gba.borrow_mut().frame();
                    show_canvas(convert_frame_to_u8(&self.gba.borrow().gpu.frame_buffer));

                    // Schedule ourself for another requestAnimationFrame callback.
                    request_animation_frame(f.borrow().as_ref().unwrap());
                }) as Box<dyn FnMut()>));

                request_animation_frame(g.borrow().as_ref().unwrap());
//                self.gba.borrow_mut().frame();
//
//                if self.follow_pc {
//                    self.follow_pc_disassemble();
//                }
//                show_canvas(convert_frame_to_u8(&self.gba.borrow().gpu.frame_buffer));
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
                            <div class="row">
                                <div class="col-3">
                                    {self.view_range_mem()}
                                </div>
                                <div class="col-9">
                                    <MemoryViewer gba={self.gba.clone()} min={self.mem_min} max={self.mem_max} initialized={self.initialized}/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

impl App {
    pub fn view_disassembly(&self) -> Html {
        let instruction_set = self.gba.borrow().cpu.get_instruction_set();
        let current_pc_num = if instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };

        if self.disassembled {
            let current_pc = self.gba.borrow().cpu.get_register(current_pc_num);

            html! {
                <div class="code-block">
                    {for (0..self.disassembly.len()).map(|val|{
                        let element = &self.disassembly[val as usize];

                        html! {
                            <div class={if self.disassembly[val as usize].address == current_pc {"disassembly-selected"} else {""}}>
                                <span class="disassembly-address">{format!("{:08X}", element.address)}</span>
                                <span class="disassembly-hex">{format!("{:08X}", element.instruction_hex)}</span>
                                <span class="disassembly-asm">{format!("{}", element.instruction_asm)}</span>
                            </div>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div class="code-block">{"Run Disassembly"}</div>
            }
        }
    }

    pub fn convert_rgb15_rgb24(value: u16) -> u32 {
        let r = ((value & 0x1F) as u32) * 255 / 31;
        let g = (((value >> 5) & 0x1F) as u32) * 255 / 31;
        let b = (((value >> 10) & 0x1F) as u32) * 255 / 31;
        (r << 16) | (g << 8) | (b)
    }

    pub fn view_bg_palette(&self) -> Html {
        html! {
            <div>
                {for (0x500_0000u32..0x500_01FFu32).step_by(32).map(|val|{
                    html! {
                        <div style="line-height: 0">
                            {for (0..32).step_by(2).map(|offset|{
                                html!{
                                    <div class="palette_block" style={format!("background: #{:06X};", App::convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(val + offset)))}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        }
    }

    pub fn view_obj_palette(&self) -> Html {
        html! {
            <div>
                {for (0x500_0200u32..0x500_03FFu32).step_by(32).map(|val|{
                    html! {
                        <div style="line-height: 0">
                            {for (0..32).step_by(2).map(|offset|{
                                html!{
                                    <div class="palette_block" style={format!("background: #{:06X};", App::convert_rgb15_rgb24(self.gba.borrow().memory_bus.mem_map.read_u16(val + offset)))}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        }
    }

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

    pub fn view_canvas(&self) -> Html {
        html! {
            <>
                <canvas id="gba-canvas"></canvas>
                <img id="gba-img" style="display:none;"></img>
            </>
        }
    }

    pub fn view_range_mem(&self) -> Html {
        html! {
            <>
                <h5>{"Memory"}</h5>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="lower-addon-mem">{"Lower"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="0" oninput=self.link.callback(|e: InputData| {Msg::UpdateInputString(e.value, RangeUpdate::MemoryViewerMin)})/>
                </div>
                <div class="input-group input-group-sm mb-3">
                    <div class="input-group-prepend">
                        <span class="input-group-text" id="upper-addon-mem">{"Upper"}</span>
                    </div>
                    <input type="text" class="form-control" placeholder="100" oninput=self.link.callback(|e: InputData| {Msg::UpdateInputString(e.value, RangeUpdate::MemoryViewerMax)})/>
                </div>
                <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::UpdateRange(RangeUpdate::MemoryViewerMax)})>{"Search"}</button>
            </>
        }
    }

    pub fn view_control(&self) -> Html {
        html! {
            <>
                // <h4>{"Control"}</h4>
                <div class="col-xs-12 col-md-6 col-xl-3">                               
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text" id="inputGroupFileAddon01">{"Bios"}</span>
                        </div>
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" id="inputGroupFile01" aria-describedby="inputGroupFileAddon01" onchange=self.link.callback(move |value| {
                                let mut result = Vec::new();
                                if let ChangeData::Files(files) = value {
                                    for x in 0..files.length() {
                                        result.push(files.get(x).unwrap())
                                    }
                                }
                                Msg::Files(result, false)
                            })/>
                            <label class="custom-file-label" for="inputGroupFile01">{format!("{}", self.bios_name)}</label>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-6 col-xl-3">                               
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text" id="inputGroupFileAddon02">{"Rom"}</span>
                        </div>
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" id="inputGroupFile02" aria-describedby="inputGroupFileAddon02" onchange=self.link.callback(|value| {
                                let mut result = Vec::new();
                                if let ChangeData::Files(files) = value {
                                    for x in 0..files.length() {
                                        result.push(files.get(x).unwrap())
                                    }
                                }
                                Msg::Files(result, true)
                            })/>
                            <label class="custom-file-label" for="inputGroupFile02">{format!("{}", self.rom_name)}</label>
                        </div>
                    </div>
                </div>
            
                <div class="col-xs-12 col-xl-3 sticky-top">
                    <div class="btn-group" role="group">
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Init})>{"Init Emulator"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Step(1)})>{"Step"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Frame})>{"Frame"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Go})>{"Run"}</button>
                    </div>
                </div>

                <div class="col-xs-12 col-xl-3 sticky-top">
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <button class="btn btn-outline-primary" type="button" onclick=self.link.callback(|_|{Msg::StartRun})>{"Run"}</button>
                        </div>
                        <input type="text" class="form-control" placeholder="080000D4" oninput=self.link.callback(|e: InputData| {Msg::UpdateRunString(e.value)})/>
                    </div>
                </div>
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


    fn follow_pc_disassemble(&mut self) {
        // Update the disassembly with the given pc follow range
        self.disassembly.clear();
        let current_pc = if self.gba.borrow().cpu.get_instruction_set() == InstructionSet::Arm { self.gba.borrow().cpu.get_register(ARM_PC) } else { self.gba.borrow().cpu.get_register(THUMB_PC) };
        let current_instruction_size = if self.gba.borrow().cpu.get_instruction_set() == InstructionSet::Arm { 4 } else { 2 };

        let mut address = current_pc as i64 + (FOLLOW_MIN * current_instruction_size);
        let total_bytes = (FOLLOW_MAX * current_instruction_size - FOLLOW_MIN * current_instruction_size) as u32;

        if address < 0 {
            address = 0;
        }

        self.disassemble(address as u32, total_bytes);
    }

    fn disassemble(&mut self, address: u32, total_bytes: u32) {
        let memory_block = self.gba.borrow().memory_bus.mem_map.read_block(address as u32, total_bytes);
        match self.gba.borrow().cpu.get_instruction_set() {
            InstructionSet::Arm => {
                for i in (0..memory_block.len()).step_by(4) {
                    let instruction: u32 = memory_block[i] as u32 |
                        ((memory_block[i as usize + 1] as u32) << 8) |
                        ((memory_block[i as usize + 2] as u32) << 16) |
                        ((memory_block[i as usize + 3] as u32) << 24);

                    let decode_result = self.gba.borrow().cpu.decode(instruction);
                    match decode_result {
                        Ok(decoded_instruction) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction,
                                instruction_asm: decoded_instruction.asm(),
                            });
                        }
                        Err(_) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction,
                                instruction_asm: "???".to_string(),
                            });
                        }
                    }
                }

                self.disassembled = true;
            }
            InstructionSet::Thumb => {
                for i in (0..memory_block.len()).step_by(2) {
                    let instruction: u16 = memory_block[i] as u16 | ((memory_block[i as usize + 1] as u16) << 8);

                    let decode_result = self.gba.borrow().cpu.decode(instruction as u32);
                    match decode_result {
                        Ok(decoded_instruction) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction as u32,
                                instruction_asm: decoded_instruction.asm(),
                            });
                        }
                        Err(_) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction as u32,
                                instruction_asm: "???".to_string(),
                            });
                        }
                    }
                }

                self.disassembled = true;
            }
        }
    }
}

pub fn convert_frame_to_u8(vec: &Vec<u32>) -> Vec<u8> {
    let mut new_vec: Vec<u8> = Vec::new();
    for i in 0..vec.len() {
        new_vec.push(((vec[i] & 0xFF_00_00) >> 16) as u8);
        new_vec.push(((vec[i] & 0xFF_00) >> 8) as u8);
        new_vec.push((vec[i] & 0xFF) as u8);
        new_vec.push(0xFF as u8);
    }
    new_vec
}