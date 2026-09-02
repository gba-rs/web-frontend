use yew::prelude::*;
use yew::{html, Component, Context, Html};

use gba_emulator::gba::GBA;
use gba_emulator::gamepak::GamePack;

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use log::{info, error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use gloo_file::callbacks::{read_as_bytes, FileReader};
use web_sys::InputEvent;
use idb::Database;

use crate::audio::player::AudioPlayer;
use crate::audio::push_audio_samples;
use crate::components::canvas::*;
use crate::dom_util::{files_from_input, input_value};
use crate::frame_pacing::{FrameAccumulator, MAX_CATCHUP_FRAMES, MAX_CATCHUP_FRAMES_TURBO, TURBO_MULTIPLIER};
use crate::logging;
use crate::save_state;
use crate::storage;

const START_PC: u32 = 0;

pub struct PlayerApp {
    file_readers: HashMap<String, FileReader>,
    bios_name: String,
    rom_name: String,
    gba: Rc<RefCell<GBA>>,
    game_pack: GamePack,
    initialized: bool,
    playing: Rc<Cell<bool>>,
    turbo: Rc<Cell<bool>>,
    keydown_closure: Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
    keyup_closure: Option<Closure<dyn FnMut(web_sys::KeyboardEvent)>>,
    audio_player: Rc<RefCell<Option<AudioPlayer>>>,
    db: Rc<RefCell<Option<Database>>>,
    save_state_slot_str: String,
}

#[derive(Clone, Copy)]
pub enum FileLoadType {
    Bios,
    Rom,
}

pub enum Msg {
    LoadedBios(String, Vec<u8>),
    LoadedRom(String, Vec<u8>),
    Files(Option<web_sys::FileList>, FileLoadType),
    Play,
    Pause,
    UpdateSaveSlot(String),
    SaveState,
    LoadState,
    StateLoaded(GBA),
    BatterySaveLoaded(Vec<u8>),
}

impl Component for PlayerApp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        info!("Created Player");

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

        PlayerApp {
            file_readers: HashMap::new(),
            bios_name: "Choose BIOS file".to_string(),
            rom_name: "Choose ROM file".to_string(),
            gba: Rc::new(RefCell::new(GBA::default())),
            game_pack: GamePack::default(),
            initialized: false,
            playing: Rc::new(Cell::new(false)),
            turbo: Rc::new(Cell::new(false)),
            keydown_closure: None,
            keyup_closure: None,
            audio_player: Rc::new(RefCell::new(None)),
            db,
            save_state_slot_str: "1".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedBios(name, bytes) => {
                self.file_readers.remove(&name);
                self.game_pack.bios = bytes;
                self.bios_name = name;
                self.initialized = false;
                true
            }
            Msg::LoadedRom(name, bytes) => {
                self.file_readers.remove(&name);
                self.game_pack.rom = bytes;
                self.rom_name = name;
                self.initialized = false;
                true
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
                                FileLoadType::Bios => Msg::LoadedBios(name_for_msg, bytes),
                                FileLoadType::Rom => Msg::LoadedRom(name_for_msg, bytes),
                            };
                            link.send_message(msg);
                        });
                        self.file_readers.insert(name, task);
                    }
                }
                false
            }
            Msg::Play => {
                if self.game_pack.bios.is_empty() || self.game_pack.rom.is_empty() {
                    error!("Choose a BIOS and ROM file first");
                    return false;
                }

                if !self.initialized {
                    self.game_pack.backup_type = GamePack::detect_backup_type(&self.game_pack.rom);
                    self.gba = Rc::new(RefCell::new(GBA::new(START_PC, &self.game_pack)));
                    self.initialized = true;
                    clear_canvas();

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

                    self.gba.borrow_mut().key_status.set_register(0xFFFF);

                    if !self.game_pack.rom.is_empty() {
                        let db = self.db.clone();
                        let rom = self.game_pack.rom.clone();
                        let link = ctx.link().clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let db_ref = db.borrow();
                            if let Some(database) = db_ref.as_ref() {
                                let key = save_state::battery_save_key(&rom);
                                match storage::get_bytes(database, &key).await {
                                    Ok(Some(bytes)) => link.send_message(Msg::BatterySaveLoaded(bytes)),
                                    Ok(None) => {}
                                    Err(e) => error!("Failed to load battery save: {:?}", e),
                                }
                            }
                        });
                    }
                } else if let Some(player) = self.audio_player.borrow().as_ref() {
                    if let Err(e) = player.resume() {
                        error!("Failed to resume audio context: {:?}", e);
                    }
                }

                if let Some(closure) = self.keydown_closure.take() {
                    let _ = window().remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                }
                if let Some(closure) = self.keyup_closure.take() {
                    let _ = window().remove_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref());
                }

                let running_down = self.playing.clone();
                let running_up = self.playing.clone();
                let gba_down = self.gba.clone();
                let gba_up = self.gba.clone();
                let turbo_down = self.turbo.clone();
                let turbo_up = self.turbo.clone();

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

                self.playing.set(true);

                let gba_clone = self.gba.clone();
                let playing = self.playing.clone();
                let turbo = self.turbo.clone();
                let audio_player = self.audio_player.clone();
                let recent_audio_samples = Rc::new(RefCell::new(Vec::new()));
                let f = Rc::new(RefCell::new(None));
                let g = f.clone();

                let mut accumulator = FrameAccumulator::new();
                let mut last_time = window().performance().unwrap().now();

                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    if !playing.get() {
                        let _ = f.borrow_mut().take();
                        return;
                    }

                    let now = window().performance().unwrap().now();
                    let mut dt = (now - last_time) / 1000.0;
                    last_time = now;

                    let is_turbo = turbo.get();
                    if is_turbo {
                        dt *= TURBO_MULTIPLIER;
                    }

                    let max_frames = if is_turbo { MAX_CATCHUP_FRAMES_TURBO } else { MAX_CATCHUP_FRAMES };
                    let frame_count = accumulator.frames_to_run(dt, max_frames);
                    for _ in 0..frame_count {
                        gba_clone.borrow_mut().frame();
                        if is_turbo {
                            let _ = std::mem::take(&mut gba_clone.borrow_mut().apu.sample_buffer);
                        } else {
                            push_audio_samples(&gba_clone, &audio_player, &recent_audio_samples);
                        }
                    }
                    if frame_count > 0 {
                        show_canvas(convert_frame_to_u8(&gba_clone.borrow().gpu.frame_buffer));
                    }

                    request_animation_frame(f.borrow().as_ref().unwrap());
                }) as Box<dyn FnMut()>));

                request_animation_frame(g.borrow().as_ref().unwrap());
                true
            }
            Msg::Pause => {
                self.playing.set(false);

                if self.initialized && !self.game_pack.rom.is_empty() {
                    let db = self.db.clone();
                    let rom = self.game_pack.rom.clone();
                    let save_data = self.gba.borrow().get_save_data();
                    wasm_bindgen_futures::spawn_local(async move {
                        let db_ref = db.borrow();
                        if let Some(database) = db_ref.as_ref() {
                            let key = save_state::battery_save_key(&rom);
                            if let Err(e) = storage::put_bytes(database, &key, &save_data).await {
                                error!("Failed to persist battery save: {:?}", e);
                            }
                        }
                    });
                }

                true
            }
            Msg::UpdateSaveSlot(value) => {
                self.save_state_slot_str = value;
                false
            }
            Msg::SaveState => {
                let slot = self.save_state_slot_str.parse::<u8>().unwrap_or(1).clamp(1, save_state::SAVE_STATE_SLOTS);
                let db = self.db.clone();
                let rom = self.game_pack.rom.clone();
                let bytes = save_state::serialize_gba(&self.gba.borrow());
                match bytes {
                    Ok(bytes) => {
                        wasm_bindgen_futures::spawn_local(async move {
                            let db_ref = db.borrow();
                            if let Some(database) = db_ref.as_ref() {
                                let key = save_state::save_state_key(&rom, slot);
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
                let slot = self.save_state_slot_str.parse::<u8>().unwrap_or(1).clamp(1, save_state::SAVE_STATE_SLOTS);
                let db = self.db.clone();
                let rom = self.game_pack.rom.clone();
                let bios = self.game_pack.bios.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let db_ref = db.borrow();
                    if let Some(database) = db_ref.as_ref() {
                        let key = save_state::save_state_key(&rom, slot);
                        match storage::get_bytes(database, &key).await {
                            Ok(Some(bytes)) => {
                                match save_state::deserialize_gba(&bytes, &bios, &rom) {
                                    Ok(gba) => link.send_message(Msg::StateLoaded(gba)),
                                    Err(e) => error!("Failed to deserialize save state: {:?}", e),
                                }
                            }
                            Ok(None) => info!("No save state in slot {}", slot),
                            Err(e) => error!("Failed to load save state: {:?}", e),
                        }
                    }
                });
                false
            }
            Msg::StateLoaded(gba) => {
                self.gba = Rc::new(RefCell::new(gba));
                true
            }
            Msg::BatterySaveLoaded(bytes) => {
                self.gba.borrow_mut().load_save_file(&bytes);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ready = !self.game_pack.bios.is_empty() && !self.game_pack.rom.is_empty();

        html! {
            <div class="player-shell">
                <header class="player-header">
                    <span class="player-logo">{"GBA"}</span>
                    <a class="player-nav-link" href="index.html">{"Debugger"}</a>
                </header>

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

                        <canvas id="gba-canvas" class="player-canvas"></canvas>
                        <canvas id="gba-canvas2" class="player-canvas-hidden"></canvas>

                        <div class="transport-row">
                            <button class="play-button" disabled={!ready} onclick={ctx.link().callback(|_| Msg::Play)}>{"Play"}</button>
                            <button class="pause-button" onclick={ctx.link().callback(|_| Msg::Pause)}>{"Pause"}</button>
                        </div>

                        <div class="save-row">
                            <span class="save-row-label">{"Slot"}</span>
                            <input class="slot-input" type="text" value={self.save_state_slot_str.clone()} oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateSaveSlot(input_value(&e)))}/>
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
            </div>
        }
    }
}
