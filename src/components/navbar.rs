use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct NavBar {}

pub enum Msg {}

impl Component for NavBar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavBar {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <a class="navbar-brand" href="#">{"GBA Emu"}</a>
                <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="io-reg-section-body collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Emulator"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Debugger"}<span class="sr-only">{"(current)"}</span></a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}