use yew::prelude::*;
use yew::{html, Component, Context, Html};

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
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use log::{info, error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo_file::callbacks::{read_as_bytes, FileReader};
use web_sys::InputEvent;

use crate::components::{
    registers::Registers,
    cpsr::Cpsr,
    status::Status,
    memory_viewer::MemoryViewer,
    io_reg::IORegisters,
    disassembler::DisassemblyElement,
    canvas::*,
    sound_panel::SoundPanel,
    sprites_panel::SpritesPanel,
    tiles_panel::TilesPanel,
    backgrounds_panel::BackgroundsPanel,
};

use crate::logging;
use crate::audio::player::AudioPlayer;
use crate::audio::push_audio_samples;
use crate::dom_util::files_from_input;
use crate::route::Route;
use crate::save_state;
use crate::storage;
use crate::frame_pacing::{FrameAccumulator, MAX_CATCHUP_FRAMES, MAX_CATCHUP_FRAMES_TURBO, TURBO_MULTIPLIER};
use idb::Database;
use yew_router::prelude::*;

pub const START_PC: u32 = 0;

pub struct App {
    pub file_readers: HashMap<String, FileReader>,
    pub rom_name: String,
    pub bios_name: String,
    pub save_name: String,
    pub save: Vec<u8>,
    pub disassembly: Vec<DisassemblyElement>,
    pub gba: Rc<RefCell<GBA>>,
    pub game_pack: GamePack,
    pub hex: bool,
    pub follow_pc: bool,
    pub initialized: bool,
    pub disassembled: bool,
    pub logging_enabled: bool,
    pub dis_min: u32,
    pub dis_max: u32,
    pub dis_min_str: String,
    pub dis_max_str: String,
    pub run_addr_str: String,
    pub active_menu: ActiveMenu,
    pub running: Rc<Cell<bool>>,
    pub turbo: Rc<Cell<bool>>,
    keydown_closure: Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
    keyup_closure: Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
    audio_player: Rc<RefCell<Option<AudioPlayer>>>,
    db: Rc<RefCell<Option<Database>>>,
    pub save_state_slot_str: String,
    pub recent_audio_samples: Rc<RefCell<Vec<i16>>>,
    pub view_mode: ViewMode,
    pub auto_spin: bool,
    pub lid_angle: u32,
    pub reset_nonce: u32,
    rom_key: u64,
    load_generation: u64,
    run_generation: u64,
    animation_frame: Option<Closure<dyn FnMut(f64)>>,
    animation_request: Option<i32>,
    last_frame_time: Option<f64>,
    accumulator: FrameAccumulator,
}

pub enum RangeUpdate {
    DisassemblyMin,
    DisassemblyMax,
}

#[derive(Clone, Copy)]
pub enum FileLoadType {
    Rom,
    Bios,
    Save
}

pub enum Msg {
    LoadedRom(String, Vec<u8>),
    LoadedBios(String, Vec<u8>),
    LoadedSave(String, Vec<u8>),
    Init,
    Play,
    Step(u8),
    Run(u32),
    Files(Option<web_sys::FileList>, FileLoadType),
    ToggleFollow,
    ToggleMenu(ActiveMenu),
    UpdateRange(RangeUpdate),
    UpdateInputString(String, RangeUpdate),
    UpdateRunString(String),
    StartRun,
    Frame,
    Go,
    Stop,
    ToggleLog,
    UpdateSaveSlot(String),
    SaveState,
    LoadState,
    StateLoaded(u64, GBA),
    BatterySaveLoaded(u64, Vec<u8>),
    AnimationFrame(u64, f64),
    SetViewMode(ViewMode),
    ToggleAutoSpin,
    SetLidAngle(String),
    ResetView,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ViewMode {
    Flat,
    Gba,
    GbaSp,
}

impl ViewMode {
    pub fn slug(self) -> &'static str {
        match self {
            ViewMode::Flat => "flat",
            ViewMode::Gba => "gba",
            ViewMode::GbaSp => "sp",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ViewMode::Flat => "2D",
            ViewMode::Gba => "3D GBA",
            ViewMode::GbaSp => "3D GBA SP",
        }
    }
}

#[derive(PartialEq)]
pub enum ActiveMenu {
    Registers,
    IO,
    Graphics,
    Debug,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        info!("Created Application");

        let db = Rc::new(RefCell::new(None));
        {
            let db = db.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match storage::open_database().await {
                    Ok(database) => {
                        *db.borrow_mut() = Some(database);
                    }
                    Err(e) => {
                        error!("Failed to open IndexedDB: {:?}", e);
                    }
                }
            });
        }

        App {
            file_readers: HashMap::new(),
            rom_name: "Choose File".to_string(),
            bios_name: "Choose File".to_string(),
            save_name: "Choose File (Optional)".to_string(),
            save: vec![],
            disassembly: vec![],
            gba: Rc::new(RefCell::new(GBA::default())),
            game_pack: GamePack::default(),
            hex: false,
            follow_pc: true,
            initialized: false,
            disassembled: false,
            logging_enabled: false,
            dis_min: 0,
            dis_max: 100,
            dis_min_str: "".to_string(),
            dis_max_str: "".to_string(),
            run_addr_str: "".to_string(),
            active_menu: ActiveMenu::Registers,
            running: Rc::new(Cell::new(false)),
            turbo: Rc::new(Cell::new(false)),
            keydown_closure: None,
            keyup_closure: None,
            audio_player: Rc::new(RefCell::new(None)),
            db,
            save_state_slot_str: "1".to_string(),
            recent_audio_samples: Rc::new(RefCell::new(Vec::new())),
            view_mode: ViewMode::Flat,
            auto_spin: true,
            lid_angle: 116,
            reset_nonce: 0,
            rom_key: 0,
            load_generation: 0,
            run_generation: 0,
            animation_frame: None,
            animation_request: None,
            last_frame_time: None,
            accumulator: FrameAccumulator::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedRom(name, bytes) => {
                self.file_readers.remove(&name);
                self.stop_animation();
                self.load_generation = self.load_generation.wrapping_add(1);
                self.rom_key = save_state::rom_content_key(&bytes);
                self.game_pack = GamePack::from_bytes(bytes, std::mem::take(&mut self.game_pack.bios));
                self.save.clear();
                self.rom_name = name;
                self.disassembled = false;
                self.initialized = false;
                true
            }
            Msg::LoadedBios(name, bytes) => {
                self.file_readers.remove(&name);
                self.stop_animation();
                self.load_generation = self.load_generation.wrapping_add(1);
                self.game_pack.bios = bytes;
                self.bios_name = name;
                self.initialized = false;
                true
            }
            Msg::LoadedSave(name, bytes) => {
                self.file_readers.remove(&name);
                self.save = bytes;
                self.save_name = name;
                true
            }
            Msg::Init => {
                self.stop_animation();
                self.load_generation = self.load_generation.wrapping_add(1);
                *self.gba.borrow_mut() = GBA::new(START_PC, &self.game_pack);
                if self.save.len() != 0 {
                    self.gba.borrow_mut().load_save_file(&self.save);
                }
                self.initialized = true;
                clear_canvas();
                info!("Created new Emulator");

                match AudioPlayer::new(gba_emulator::apu::OUTPUT_SAMPLE_RATE as u32) {
                    Ok(player) => {
                        if let Err(e) = player.resume() {
                            error!("Failed to resume audio context: {:?}", e);
                        }
                        *self.audio_player.borrow_mut() = Some(player);
                    }
                    Err(e) => {
                        error!("Failed to create audio context: {:?}", e);
                        *self.audio_player.borrow_mut() = None;
                    }
                }

                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

                self.gba.borrow_mut().key_status.set_register(0xFFFF);

                if let Some(closure) = self.keydown_closure.take() {
                    let _ = window().remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }
                if let Some(closure) = self.keyup_closure.take() {
                    let _ = window().remove_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
                }

                let running_down = self.running.clone();
                let running_up = self.running.clone();
                let gba_down = self.gba.clone();
                let gba_up = self.gba.clone();
                let turbo_down = self.turbo.clone();
                let turbo_up = self.turbo.clone();

                info!("Setup key bindings");
                let key_down = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                    if running_down.get() {
                        match event.code().as_str() {
                            "KeyW" => gba_down.borrow_mut().key_status.set_dpad_up(0),
                            "KeyS" => gba_down.borrow_mut().key_status.set_dpad_down(0),
                            "KeyA" => gba_down.borrow_mut().key_status.set_dpad_left(0),
                            "KeyD" => gba_down.borrow_mut().key_status.set_dpad_right(0),
                            "KeyH" => gba_down.borrow_mut().key_status.set_button_a(0),
                            "KeyJ" => gba_down.borrow_mut().key_status.set_button_b(0),
                            "KeyE" => gba_down.borrow_mut().key_status.set_button_r(0),
                            "KeyQ" => gba_down.borrow_mut().key_status.set_button_l(0),
                            "Backspace" => gba_down.borrow_mut().key_status.set_button_select(0),
                            "Enter" => gba_down.borrow_mut().key_status.set_button_start(0),
                            "Space" => turbo_down.set(true),
                            _ => {}
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let key_up = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                    if running_up.get() {
                        match event.code().as_str() {
                            "KeyW" => gba_up.borrow_mut().key_status.set_dpad_up(1),
                            "KeyS" => gba_up.borrow_mut().key_status.set_dpad_down(1),
                            "KeyA" => gba_up.borrow_mut().key_status.set_dpad_left(1),
                            "KeyD" => gba_up.borrow_mut().key_status.set_dpad_right(1),
                            "KeyH" => gba_up.borrow_mut().key_status.set_button_a(1),
                            "KeyJ" => gba_up.borrow_mut().key_status.set_button_b(1),
                            "KeyE" => gba_up.borrow_mut().key_status.set_button_r(1),
                            "KeyQ" => gba_up.borrow_mut().key_status.set_button_l(1),
                            "Backspace" => gba_up.borrow_mut().key_status.set_button_select(1),
                            "Enter" => gba_up.borrow_mut().key_status.set_button_start(1),
                            "Space" => turbo_up.set(false),
                            _ => {}
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = window().add_event_listener_with_callback("keydown", key_down.as_ref().unchecked_ref());
                let _ = window().add_event_listener_with_callback("keyup", key_up.as_ref().unchecked_ref());

                self.keydown_closure = Some(key_down);
                self.keyup_closure = Some(key_up);

                if self.save.is_empty() && !self.game_pack.rom.is_empty() {
                    let db = self.db.clone();
                    let rom_key = self.rom_key;
                    let generation = self.load_generation;
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let db_ref = db.borrow();
                        if let Some(database) = db_ref.as_ref() {
                            let key = save_state::battery_save_key_from_id(rom_key);
                            match storage::get_bytes(database, &key).await {
                                Ok(Some(bytes)) => link.send_message(Msg::BatterySaveLoaded(generation, bytes)),
                                Ok(None) => {}
                                Err(e) => error!("Failed to load battery save: {:?}", e),
                            }
                        }
                    });
                }

                true
            }
            Msg::Play => {
                if !self.initialized {
                    ctx.link().send_message(Msg::Init);
                }
                ctx.link().send_message(Msg::Go);
                false
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
                push_audio_samples(&self.gba, &self.audio_player, &self.recent_audio_samples);

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
                let result = u32::from_str_radix(&self.run_addr_str, 16);
                match result {
                    Ok(val) => {
                        ctx.link().send_message(Msg::Run(val));
                    }
                    Err(_) => {
                        error!("Error parsing run address");
                    }
                }

                false
            }
            Msg::ToggleLog => {
                self.logging_enabled = !self.logging_enabled;
                logging::set_logging_enabled(self.logging_enabled);
                true
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
                if let Some(file_list) = files {
                    for file in gloo_file::FileList::from(file_list).iter() {
                        let name = file.name();
                        let link = ctx.link().clone();
                        let name_for_msg = name.clone();
                        let task = read_as_bytes(file, move |res| {
                            let bytes = res.expect("failed to read file");
                            let msg = match file_type {
                                FileLoadType::Rom => Msg::LoadedRom(name_for_msg, bytes),
                                FileLoadType::Bios => Msg::LoadedBios(name_for_msg, bytes),
                                FileLoadType::Save => Msg::LoadedSave(name_for_msg, bytes),
                            };
                            link.send_message(msg);
                        });
                        self.file_readers.insert(name, task);
                    }
                }
                false
            }
            Msg::ToggleMenu(menu_item) => {
                self.active_menu = menu_item;
                true
            },
            Msg::Go => {
                if !self.initialized || self.running.replace(true) { return false; }
                self.run_generation = self.run_generation.wrapping_add(1);
                let generation = self.run_generation;
                let link = ctx.link().clone();
                self.animation_frame = Some(Closure::wrap(Box::new(move |time: f64| {
                    link.send_message(Msg::AnimationFrame(generation, time));
                }) as Box<dyn FnMut(f64)>));
                self.last_frame_time = None;
                self.accumulator = FrameAccumulator::new();
                self.schedule_frame();
                true
            },
            Msg::AnimationFrame(generation, now) => {
                if !self.running.get() || generation != self.run_generation { return false; }
                self.animation_request = None;
                let mut dt = self.last_frame_time.map(|last| (now - last) / 1000.0).unwrap_or(0.0);
                self.last_frame_time = Some(now);
                let turbo = self.turbo.get();
                if turbo { dt *= TURBO_MULTIPLIER; }
                let max = if turbo { MAX_CATCHUP_FRAMES_TURBO } else { MAX_CATCHUP_FRAMES };
                let count = self.accumulator.frames_to_run(dt, max);
                for _ in 0..count {
                    self.gba.borrow_mut().frame();
                    if turbo { self.gba.borrow_mut().apu.sample_buffer.clear(); }
                    else { push_audio_samples(&self.gba, &self.audio_player, &self.recent_audio_samples); }
                }
                if count > 0 { show_canvas(convert_frame_to_u8(&self.gba.borrow().gpu.frame_buffer)); }
                self.schedule_frame();
                false
            },
            Msg::Stop => {
                self.stop_animation();
                if self.follow_pc {
                    self.follow_pc_disassemble();
                }

                if self.initialized && !self.game_pack.rom.is_empty() {
                    let db = self.db.clone();
                    let rom_key = self.rom_key;
                    let save_data = self.gba.borrow().get_save_data();
                    wasm_bindgen_futures::spawn_local(async move {
                        let db_ref = db.borrow();
                        if let Some(database) = db_ref.as_ref() {
                            let key = save_state::battery_save_key_from_id(rom_key);
                            if let Err(e) = storage::put_bytes(database, &key, &save_data).await {
                                error!("Failed to persist battery save: {:?}", e);
                            }
                        }
                    });
                }

                true
            },
            Msg::UpdateSaveSlot(value) => {
                self.save_state_slot_str = value;
                false
            }
            Msg::SaveState => {
                let slot = self.save_state_slot_str.parse::<u8>().unwrap_or(1).clamp(1, save_state::SAVE_STATE_SLOTS);
                let db = self.db.clone();
                let rom_key = self.rom_key;
                let bytes = save_state::serialize_gba(&self.gba.borrow());
                match bytes {
                    Ok(bytes) => {
                        wasm_bindgen_futures::spawn_local(async move {
                            let db_ref = db.borrow();
                            if let Some(database) = db_ref.as_ref() {
                                let key = save_state::save_state_key_from_id(rom_key, slot);
                                if let Err(e) = storage::put_bytes(database, &key, &bytes).await {
                                    error!("Failed to save state: {:?}", e);
                                }
                            }
                        });
                    }
                    Err(e) => error!("Failed to serialize save state: {:?}", e),
                }
                false
            }
            Msg::LoadState => {
                self.stop_animation();
                self.load_generation = self.load_generation.wrapping_add(1);
                let generation = self.load_generation;
                let rom_key = self.rom_key;
                let slot = self.save_state_slot_str.parse::<u8>().unwrap_or(1).clamp(1, save_state::SAVE_STATE_SLOTS);
                let db = self.db.clone();
                let rom = self.game_pack.rom.clone();
                let bios = self.game_pack.bios.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let db_ref = db.borrow();
                    if let Some(database) = db_ref.as_ref() {
                        let key = save_state::save_state_key_from_id(rom_key, slot);
                        match storage::get_bytes(database, &key).await {
                            Ok(Some(bytes)) => {
                                match save_state::deserialize_gba(&bytes, &bios, &rom) {
                                    Ok(gba) => link.send_message(Msg::StateLoaded(generation, gba)),
                                    Err(e) => error!("Failed to deserialize save state: {:?}", e),
                                }
                            }
                            Ok(None) => info!("No save state in slot {}", slot),
                            Err(e) => error!("Failed to load save state: {:?}", e),
                        }
                    }
                });
                true
            }
            Msg::StateLoaded(generation, gba) => {
                if generation != self.load_generation { return false; }
                self.stop_animation();
                *self.gba.borrow_mut() = gba;
                if self.follow_pc {
                    self.follow_pc_disassemble();
                }
                true
            }
            Msg::BatterySaveLoaded(generation, bytes) => {
                if generation != self.load_generation { return false; }
                self.gba.borrow_mut().load_save_file(&bytes);
                true
            }
            Msg::SetViewMode(mode) => {
                self.view_mode = mode;
                true
            }
            Msg::ToggleAutoSpin => {
                self.auto_spin = !self.auto_spin;
                true
            }
            Msg::SetLidAngle(value) => {
                if let Ok(v) = value.parse::<u32>() {
                    self.lid_angle = v.min(150);
                }
                true
            }
            Msg::ResetView => {
                self.reset_nonce = self.reset_nonce.wrapping_add(1);
                true
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.stop_animation();
        if let Some(closure) = self.keydown_closure.take() {
            let _ = window().remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        }
        if let Some(closure) = self.keyup_closure.take() {
            let _ = window().remove_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let play_html = self.view_play(ctx);
        let debug_html = self.view_debug(ctx);
        let switch = Callback::from(move |route: Route| -> Html {
            match route {
                Route::Play => play_html.clone(),
                Route::Debug => debug_html.clone(),
            }
        });

        html! {
            <HashRouter>
                <div class="app-shell">
                    {self.view_nav(ctx)}
                    <div class="app-canvas-wrap">
                        {self.view_canvas(ctx)}
                    </div>
                    <Switch<Route> render={switch}/>
                </div>
            </HashRouter>
        }
    }
}

impl App {
    fn schedule_frame(&mut self) {
        if let Some(callback) = &self.animation_frame {
            self.animation_request = Some(window().request_animation_frame(callback.as_ref().unchecked_ref()).expect("request animation frame"));
        }
    }

    fn stop_animation(&mut self) {
        self.running.set(false);
        self.run_generation = self.run_generation.wrapping_add(1);
        if let Some(id) = self.animation_request.take() { let _ = window().cancel_animation_frame(id); }
        self.animation_frame = None;
        self.last_frame_time = None;
        self.accumulator = FrameAccumulator::new();
    }

    pub fn view_nav(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header class="app-nav">
                <span class="app-logo">{"GBA"}</span>
                <nav class="app-nav-links">
                    <Link<Route> classes="app-nav-link" to={Route::Play}>{"Play"}</Link<Route>>
                    <Link<Route> classes="app-nav-link" to={Route::Debug}>{"Debugger"}</Link<Route>>
                </nav>
            </header>
        }
    }

    pub fn view_play(&self, ctx: &Context<Self>) -> Html {
        let ready = !self.game_pack.bios.is_empty() && !self.game_pack.rom.is_empty();

        html! {
            <main class="player-main">
                <div class="player-card">
                    <div class="file-row">
                        <label class="file-picker">
                            <span class="file-picker-label">{"BIOS"}</span>
                            <span class="file-picker-name">{self.bios_name.clone()}</span>
                            <input type="file" onchange={ctx.link().callback(|e: Event| {
                                Msg::Files(files_from_input(e), FileLoadType::Bios)
                            })}/>
                        </label>
                        <label class="file-picker">
                            <span class="file-picker-label">{"ROM"}</span>
                            <span class="file-picker-name">{self.rom_name.clone()}</span>
                            <input type="file" onchange={ctx.link().callback(|e: Event| {
                                Msg::Files(files_from_input(e), FileLoadType::Rom)
                            })}/>
                        </label>
                    </div>

                    <div class="transport-row">
                        <button class="play-button" disabled={!ready} onclick={ctx.link().callback(|_| Msg::Play)}>{"Play"}</button>
                        <button class="pause-button" onclick={ctx.link().callback(|_| Msg::Stop)}>{"Pause"}</button>
                    </div>

                    <div class="save-row">
                        <span class="save-row-label">{"Slot"}</span>
                        <input class="slot-input" type="text" value={self.save_state_slot_str.clone()} oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateSaveSlot(crate::dom_util::input_value(&e)))}/>
                        <button class="save-button" onclick={ctx.link().callback(|_| Msg::SaveState)}>{"Save"}</button>
                        <button class="save-button" onclick={ctx.link().callback(|_| Msg::LoadState)}>{"Load"}</button>
                    </div>

                    <div class="key-legend">
                        <span>{"D-Pad: WASD"}</span>
                        <span>{"A/B: H/J"}</span>
                        <span>{"L/R: Q/E"}</span>
                        <span>{"Start/Select: Enter/Backspace"}</span>
                        <span>{"Turbo: Space"}</span>
                    </div>
                </div>
            </main>
        }
    }

    pub fn view_debug(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="debug-shell">
                {self.view_control(ctx)}
                <div class="debug-columns">
                     <div class="debug-column">
                         <div class="pill-tabs">
                           <a class={format!("pill-tab {}",self.is_menu_tab_active(ActiveMenu::Registers))} href="#" onclick={ctx.link().callback(|e: MouseEvent|{e.prevent_default(); Msg::ToggleMenu(ActiveMenu::Registers)})}>{"Registers/Status"}</a>
                           <a class={format!("pill-tab {}",self.is_menu_tab_active(ActiveMenu::IO))} href="#" onclick={ctx.link().callback(|e: MouseEvent|{e.prevent_default(); Msg::ToggleMenu(ActiveMenu::IO)})}>{"IO Registers"}</a>
                           <a class={format!("pill-tab {}",self.is_menu_tab_active(ActiveMenu::Graphics))} href="#" onclick={ctx.link().callback(|e: MouseEvent|{e.prevent_default(); Msg::ToggleMenu(ActiveMenu::Graphics)})}>{"Graphics"}</a>
                           <a class={format!("pill-tab {}",self.is_menu_tab_active(ActiveMenu::Debug))} href="#" onclick={ctx.link().callback(|e: MouseEvent|{e.prevent_default(); Msg::ToggleMenu(ActiveMenu::Debug)})}>{"Debug"}</a>
                         </div>
                         <div class="debug-columns" hidden={self.active_menu != ActiveMenu::Registers}>
                             <div class="debug-column">
                                <Status gba={self.gba.clone()}/>
                                <Cpsr gba={self.gba.clone()}/>
                            </div>

                            <div class="debug-column">
                                <Registers hex={self.hex} gba={self.gba.clone()}/>
                            </div>
                         </div>
                         <div hidden={self.active_menu != ActiveMenu::IO}>
                            <IORegisters hex={self.hex} gba={self.gba.clone()}/>
                         </div>
                         <div class="debug-columns" hidden={self.active_menu != ActiveMenu::Graphics}>
                                <div class="debug-column text-center">
                                    <h5>{"Background Palette"}</h5>
                                    {self.view_bg_palette()}
                                </div>
                                <div class="debug-column text-center">
                                    <h5>{"Object Palette"}</h5>
                                    {self.view_obj_palette()}
                                </div>
                        </div>
                         <div class="debug-columns" hidden={self.active_menu != ActiveMenu::Debug}>
                            <div class="debug-column text-center">
                                <h5>{"Sound"}</h5>
                                <SoundPanel samples={self.recent_audio_samples.borrow().clone()}/>
                            </div>
                            <div class="debug-column text-center">
                                <h5>{"Sprites"}</h5>
                                <SpritesPanel gba={self.gba.clone()}/>
                            </div>
                            <div class="debug-column text-center">
                                <h5>{"Tiles"}</h5>
                                <TilesPanel gba={self.gba.clone()}/>
                            </div>
                            <div class="debug-column text-center">
                                <h5>{"Backgrounds"}</h5>
                                <BackgroundsPanel gba={self.gba.clone()}/>
                            </div>
                         </div>
                     </div>

                    <div class="debug-column">
                        <div class="debug-split">
                            <div class="debug-split-side">
                                {self.view_range_dis(ctx)}
                            </div>
                            <div class="debug-split-main">
                                {self.view_disassembly()}
                            </div>
                        </div>
                        <MemoryViewer gba={self.gba.clone()} initialized={self.initialized}/>
                    </div>
                </div>
            </div>
        }
    }

    pub fn view_range_dis(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h5>{"Disassembly"}</h5>
                <div class="field-row">
                    <span class="field-row-label">{"Lower"}</span>
                    <input type="text" class="text-input" placeholder="0" oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateInputString(crate::dom_util::input_value(&e), RangeUpdate::DisassemblyMin)})}/>
                </div>
                <div class="field-row">
                    <span class="field-row-label">{"Upper"}</span>
                    <input type="text" class="text-input" placeholder="100" oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateInputString(crate::dom_util::input_value(&e), RangeUpdate::DisassemblyMax)})}/>
                </div>
                <label class="toggle-row">
                    <span class="toggle-row-label">{"Follow PC"}</span>
                    <input type="checkbox" checked={self.follow_pc} onclick={ctx.link().callback(|_|{Msg::ToggleFollow})}/>
                </label>
                <button class="btn" onclick={ctx.link().callback(|_|{Msg::UpdateRange(RangeUpdate::DisassemblyMax)})}>{"Search"}</button>
            </>
        }
    }

    pub fn is_menu_tab_active(&self, menu_item: ActiveMenu) -> String {
        if menu_item == self.active_menu {
            return format!("active");
        }
        return format!("");
    }
}

