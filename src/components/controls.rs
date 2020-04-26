use crate::app::{App, Msg, FileLoadType};
use yew::prelude::*;
use yew::{html, InputData, Html};

impl App {
    pub fn view_control(&self) -> Html {
        html! {
            <>
                // <h4>{"Control"}</h4>
                <div class="col-xs-12 col-md-1 col-xl-2">                               
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
                                Msg::Files(result, FileLoadType::Bios)
                            })/>
                            <label class="custom-file-label" for="inputGroupFile01">{format!("{}", self.bios_name)}</label>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-1 col-xl-2">                               
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
                                Msg::Files(result, FileLoadType::Rom)
                            })/>
                            <label class="custom-file-label" for="inputGroupFile02">{format!("{}", self.rom_name)}</label>
                        </div>
                    </div>
                </div>

                <div class="col-xs-12 col-md-1 col-xl-2">                               
                <div class="input-group mb-3">
                        <div class="input-group-prepend">
                            <span class="input-group-text" id="inputGroupFileAddon01">{"Save"}</span>
                        </div>
                        <div class="custom-file">
                            <input type="file" class="custom-file-input" id="inputGroupFile01" aria-describedby="inputGroupFileAddon01" onchange=self.link.callback(move |value| {
                                let mut result = Vec::new();
                                if let ChangeData::Files(files) = value {
                                    for x in 0..files.length() {
                                        result.push(files.get(x).unwrap())
                                    }
                                }
                                Msg::Files(result, FileLoadType::Save)
                            })/>
                            <label class="custom-file-label" for="inputGroupFile01">{format!("{}", self.save_name)}</label>
                        </div>
                    </div>
                </div>
            
                <div class="col-xs-12 col-xl-4">
                    <div class="btn-group" role="group">
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Init})>{"Init Emulator"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Step(1)})>{"Step"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Frame})>{"Frame"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Go})>{"Go"}</button>
                        <button class="btn btn-outline-primary" onclick=self.link.callback(|_|{Msg::Stop})>{"Stop"}</button>
                    </div>
                </div>

                <div class="col-xs-12 col-xl-2">
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
}