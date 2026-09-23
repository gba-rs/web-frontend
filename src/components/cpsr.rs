use yew::prelude::*;
use yew::{html, Component, Context, Html};
use gba_emulator::gba::GBA;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpsr {
    props: CpsrProp
}

#[derive(Properties, Clone)]
pub struct CpsrProp {
    pub gba: Rc<RefCell<GBA>>
}

impl PartialEq for CpsrProp {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.gba, &other.gba)
    }
}

pub enum UpdateFlagType{
    Carry,
    Negative,
    SignedOverflow,
    Zero,
    FiqDisable,
    IrqDisable,
    StateBit
}

pub enum Msg {
    UpdateFlag(UpdateFlagType)
}

impl Component for Cpsr {
    type Message = Msg;
    type Properties = CpsrProp;

    fn create(ctx: &Context<Self>) -> Self {
        Cpsr {
            props: ctx.props().clone()
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let flags = self.props.gba.borrow().cpu.cpsr.flags.clone();
        let control_bits = self.props.gba.borrow().cpu.cpsr.control_bits.clone();
        match msg {
            Msg::UpdateFlag(flag_to_update) => {
                match flag_to_update {
                    UpdateFlagType::Carry => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.carry = !flags.carry;
                    },
                    UpdateFlagType::Negative => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.negative = !flags.negative;
                    },
                    UpdateFlagType::SignedOverflow => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.signed_overflow = !flags.signed_overflow;
                    },
                    UpdateFlagType::Zero => {
                        self.props.gba.borrow_mut().cpu.cpsr.flags.zero = !flags.zero;
                    },
                    UpdateFlagType::FiqDisable => {
                        self.props.gba.borrow_mut().cpu.cpsr.control_bits.fiq_disable = !control_bits.fiq_disable;
                    },
                    UpdateFlagType::IrqDisable => {
                        self.props.gba.borrow_mut().cpu.cpsr.control_bits.irq_disable = !control_bits.irq_disable;
                    },
                    UpdateFlagType::StateBit => {
                        self.props.gba.borrow_mut().cpu.cpsr.control_bits.state_bit = !control_bits.state_bit;
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let flags = self.props.gba.borrow().cpu.cpsr.flags.clone();
        let control_bits = self.props.gba.borrow().cpu.cpsr.control_bits.clone();

        html! {
            <div class="cpsr-panel">
                <h4>{"Current Program Status Register"}</h4>
                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("Carry - {:?}", flags.carry)}</span>
                    <input type="checkbox" checked={flags.carry} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::Carry)})}/>
                </label>

                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("Negative - {:?}", flags.negative)}</span>
                    <input type="checkbox" checked={flags.negative} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::Negative)})}/>
                </label>

                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("Signed Overflow - {:?}", flags.signed_overflow)}</span>
                    <input type="checkbox" checked={flags.signed_overflow} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::SignedOverflow)})}/>
                </label>

                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("Zero - {:?}", flags.zero)}</span>
                    <input type="checkbox" checked={flags.zero} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::Zero)})}/>
                </label>

                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("FIQ Disable - {:?}", control_bits.fiq_disable)}</span>
                    <input type="checkbox" checked={control_bits.fiq_disable} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::FiqDisable)})}/>
                </label>

                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("IRQ Disable - {:?}", control_bits.irq_disable)}</span>
                    <input type="checkbox" checked={control_bits.irq_disable} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::IrqDisable)})}/>
                </label>

                <label class="toggle-row">
                    <span class="toggle-row-label">{&format!("State Bit - {:?}", control_bits.state_bit)}</span>
                    <input type="checkbox" checked={control_bits.state_bit} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::StateBit)})}/>
                </label>

                <div class="toggle-row">
                    <span class="toggle-row-label">{&format!("Mode Bits - {:b}", control_bits.mode_bits)}</span>
                </div>
            </div>
        }
    }
}
