use yew::prelude::*;
use yew::{html, Component, Context, Html};
use gba_emulator::gba::GBA;
use gba_emulator::cpu::cpu::{InstructionSet, OperatingMode};
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::HtmlSelectElement;

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
        let instruction_set = self.props.gba.borrow().cpu.get_instruction_set();
        let operating_mode = self.props.gba.borrow().cpu.get_operating_mode();

        html! {
            <div class="status-panel">
                <h4>{"Status"}</h4>
                <div class="select-row">
                    <span class="select-row-label">{"Instruction Set"}</span>
                    <select class="select-control" onchange={ctx.link().callback(|e: Event| {
                        let value = e.target_unchecked_into::<HtmlSelectElement>().value();
                        Msg::UpdateInstructionSet(if value == "Thumb" { InstructionSet::Thumb } else { InstructionSet::Arm })
                    })}>
                        <option value="Arm" selected={instruction_set == InstructionSet::Arm}>{"Arm"}</option>
                        <option value="Thumb" selected={instruction_set == InstructionSet::Thumb}>{"Thumb"}</option>
                    </select>
                </div>
                <div class="select-row">
                    <span class="select-row-label">{"Operating Mode"}</span>
                    <select class="select-control" onchange={ctx.link().callback(|e: Event| {
                        let value = e.target_unchecked_into::<HtmlSelectElement>().value();
                        Msg::UpdateOperatingMode(match value.as_str() {
                            "User" => OperatingMode::User,
                            "FastInterrupt" => OperatingMode::FastInterrupt,
                            "Supervisor" => OperatingMode::Supervisor,
                            "Abort" => OperatingMode::Abort,
                            "Interrupt" => OperatingMode::Interrupt,
                            "Undefined" => OperatingMode::Undefined,
                            _ => OperatingMode::System,
                        })
                    })}>
                        <option value="System" selected={operating_mode == OperatingMode::System}>{"System"}</option>
                        <option value="User" selected={operating_mode == OperatingMode::User}>{"User"}</option>
                        <option value="FastInterrupt" selected={operating_mode == OperatingMode::FastInterrupt}>{"Fast Interrupt"}</option>
                        <option value="Supervisor" selected={operating_mode == OperatingMode::Supervisor}>{"Supervisor"}</option>
                        <option value="Abort" selected={operating_mode == OperatingMode::Abort}>{"Abort"}</option>
                        <option value="Interrupt" selected={operating_mode == OperatingMode::Interrupt}>{"Interrupt"}</option>
                        <option value="Undefined" selected={operating_mode == OperatingMode::Undefined}>{"Undefined"}</option>
                    </select>
                </div>
            </div>
        }
    }
}
