use crate::app::{App, Msg, FileLoadType};
use crate::dom_util::files_from_input;
use yew::prelude::*;
use yew::{html, Context, Html};
use web_sys::InputEvent;

impl App {
    pub fn view_control(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="debug-toolbar">
                <label class="file-field">
                    <span class="file-field-label">{"Bios"}</span>
                    <span class="file-field-name">{format!("{}", self.bios_name)}</span>
                    <input type="file" id="inputGroupFile01" onchange={ctx.link().callback(|e: Event| {
                        Msg::Files(files_from_input(e), FileLoadType::Bios)
                    })}/>
                </label>

                <label class="file-field">
                    <span class="file-field-label">{"Rom"}</span>
                    <span class="file-field-name">{format!("{}", self.rom_name)}</span>
                    <input type="file" id="inputGroupFile02" onchange={ctx.link().callback(|e: Event| {
                        Msg::Files(files_from_input(e), FileLoadType::Rom)
                    })}/>
                </label>

                <label class="file-field">
                    <span class="file-field-label">{"Save"}</span>
                    <span class="file-field-name">{format!("{}", self.save_name)}</span>
                    <input type="file" id="inputGroupFile03" onchange={ctx.link().callback(|e: Event| {
                        Msg::Files(files_from_input(e), FileLoadType::Save)
                    })}/>
                </label>

                <div class="button-group">
                    <button class="btn" onclick={ctx.link().callback(|_|{Msg::Init})}>{"Init Emulator"}</button>
                    <button class="btn" onclick={ctx.link().callback(|_|{Msg::Step(1)})}>{"Step"}</button>
                    <button class="btn" onclick={ctx.link().callback(|_|{Msg::Frame})}>{"Frame"}</button>
                    <button class="btn" onclick={ctx.link().callback(|_|{Msg::Go})}>{"Go"}</button>
                    <button class="btn" onclick={ctx.link().callback(|_|{Msg::Stop})}>{"Stop"}</button>
                </div>

                <label class="toggle-row">
                    <span class="toggle-row-label">{"Logging"}</span>
                    <input type="checkbox" checked={self.logging_enabled} onclick={ctx.link().callback(|_|{Msg::ToggleLog})}/>
                </label>

                <div class="button-group">
                    <button class="btn" type="button" onclick={ctx.link().callback(|_|{Msg::StartRun})}>{"Run"}</button>
                    <input type="text" class="text-input" placeholder="080000D4" oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateRunString(crate::dom_util::input_value(&e))})}/>
                </div>

                <div class="button-group">
                    <span class="button-group-label">{"Slot"}</span>
                    <input type="text" class="text-input text-input-narrow" value={self.save_state_slot_str.clone()} oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateSaveSlot(crate::dom_util::input_value(&e))})}/>
                    <button class="btn" type="button" onclick={ctx.link().callback(|_|{Msg::SaveState})}>{"Save State"}</button>
                    <button class="btn" type="button" onclick={ctx.link().callback(|_|{Msg::LoadState})}>{"Load State"}</button>
                </div>
            </div>
        }
    }
}
