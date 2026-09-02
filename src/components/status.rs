use yew::prelude::*;
use yew::{html, Component, Context, Html};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::{InstructionSet, OperatingMode};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Status {
    props: StatusProp,
}

#[derive(Properties, Clone)]
pub struct StatusProp {
    pub gba: Rc<RefCell<GBA>>
}

impl PartialEq for StatusProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba)
    }
}

pub enum Msg {
    UpdateInstructionSet(InstructionSet),
    UpdateOperatingMode(OperatingMode)
}

impl Component for Status {
    type Message = Msg;
    type Properties = StatusProp;

    fn create(ctx: &Context<Self>) -> Self {
        Status {
            props: ctx.props().clone(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateInstructionSet(instr_set) => {
                self.props.gba.borrow_mut().cpu.set_instruction_set(instr_set);
            },
            Msg::UpdateOperatingMode(op_mode) => {
                self.props.gba.borrow_mut().cpu.set_operating_mode(op_mode);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h4>{"Status"}</h4>
                <div class="dropdown m-2">
                    <button class="btn btn-outline-primary dropdown-toggle" type="button" data-toggle="dropdown">
                        {&format!("{:?}", self.props.gba.borrow().cpu.get_instruction_set())}
                    </button>
                    <div class="dropdown-menu">
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateInstructionSet(InstructionSet::Arm)})}>{"Arm"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateInstructionSet(InstructionSet::Thumb)})}>{"Thumb"}</button>
                    </div>
                </div>
                <div class="dropdown m-2">
                    <button class="btn btn-outline-primary dropdown-toggle" type="button" data-toggle="dropdown">
                        {&format!("{:?}", self.props.gba.borrow().cpu.get_operating_mode())}
                    </button>
                    <div class="dropdown-menu">
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::System)})}>{"System"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::User)})}>{"User"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::FastInterrupt)})}>{"Fast Interrupt"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::Supervisor)})}>{"Supervisor"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::Abort)})}>{"Abort"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::Interrupt)})}>{"Interrupt"}</button>
                        <button class="dropdown-item" type="button" onclick={ctx.link().callback(|_|{Msg::UpdateOperatingMode(OperatingMode::Undefined)})}>{"Undefined"}</button>
                    </div>
                </div>
            </div>
        }
    }
}
