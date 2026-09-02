use crate::app::{files_from_input, App, Msg, FileLoadType};
use yew::prelude::*;
use yew::{html, Context, Html};
use web_sys::InputEvent;

impl App {
    pub fn view_control(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="col-xs-12 col-md-2 col-xl-2">
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text" id="inputGroupFileAddon01">{"Bios"}</span>
                        </div>
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" id="inputGroupFile01" aria-describedby="inputGroupFileAddon01" onchange={ctx.link().callback(|e: Event| {
                                Msg::Files(files_from_input(e), FileLoadType::Bios)
                            })}/>
                            <label class="custom-file-label" for="inputGroupFile01">{format!("{}", self.bios_name)}</label>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-2 col-xl-2">
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text" id="inputGroupFileAddon02">{"Rom"}</span>
                        </div>
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" id="inputGroupFile02" aria-describedby="inputGroupFileAddon02" onchange={ctx.link().callback(|e: Event| {
                                Msg::Files(files_from_input(e), FileLoadType::Rom)
                            })}/>
                            <label class="custom-file-label" for="inputGroupFile02">{format!("{}", self.rom_name)}</label>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-2 col-xl-2">
                <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text" id="inputGroupFileAddon03">{"Save"}</span>
                        </div>
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" id="inputGroupFile03" aria-describedby="inputGroupFileAddon03" onchange={ctx.link().callback(|e: Event| {
                                Msg::Files(files_from_input(e), FileLoadType::Save)
                            })}/>
                            <label class="custom-file-label" for="inputGroupFile03">{format!("{}", self.save_name)}</label>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-4 col-xl-4">
                    <div class="btn-group" role="group">
                        <button class="btn btn-outline-primary" onclick={ctx.link().callback(|_|{Msg::Init})}>{"Init Emulator"}</button>
                        <button class="btn btn-outline-primary" onclick={ctx.link().callback(|_|{Msg::Step(1)})}>{"Step"}</button>
                        <button class="btn btn-outline-primary" onclick={ctx.link().callback(|_|{Msg::Frame})}>{"Frame"}</button>
                        <button class="btn btn-outline-primary" onclick={ctx.link().callback(|_|{Msg::Go})}>{"Go"}</button>
                        <button class="btn btn-outline-primary" onclick={ctx.link().callback(|_|{Msg::Stop})}>{"Stop"}</button>

                        <div class="input-group-prepend">
                            <span class="input-group-text" id="follow-addon">{"Logging"}</span>
                            <div class="input-group-text">
                                <input type="checkbox" checked={self.logging_enabled} onclick={ctx.link().callback(|_|{Msg::ToggleLog})}/>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-4 col-xl-2">
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <button class="btn btn-outline-primary" type="button" onclick={ctx.link().callback(|_|{Msg::StartRun})}>{"Run"}</button>
                        </div>
                        <input type="text" class="form-control" placeholder="080000D4" oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateRunString(crate::dom_util::input_value(&e))})}/>
                    </div>
                </div>

                <div class="col-xs-12 col-md-4 col-xl-2">
                    <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text">{"Slot"}</span>
                        </div>
                        <input type="text" class="form-control" style="max-width: 3rem;" value={self.save_state_slot_str.clone()} oninput={ctx.link().callback(|e: InputEvent| {Msg::UpdateSaveSlot(crate::dom_util::input_value(&e))})}/>
                        <button class="btn btn-outline-primary" type="button" onclick={ctx.link().callback(|_|{Msg::SaveState})}>{"Save State"}</button>
                        <button class="btn btn-outline-primary" type="button" onclick={ctx.link().callback(|_|{Msg::LoadState})}>{"Load State"}</button>
                    </div>
                </div>
            </>
        }
    }
}
