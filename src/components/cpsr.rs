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
            <div class="col-12">
                <h4>{"Current Program Status Register"}</h4>
                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Carry - {:?}", flags.carry)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.carry} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::Carry)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Negative - {:?}", flags.negative)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.negative} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::Negative)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Signed Overflow - {:?}", flags.signed_overflow)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.signed_overflow} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::SignedOverflow)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Zero - {:?}", flags.zero)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={flags.zero} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::Zero)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("FIQ Disable - {:?}", control_bits.fiq_disable)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={control_bits.fiq_disable} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::FiqDisable)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("IRQ Disable - {:?}", control_bits.irq_disable)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={control_bits.irq_disable} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::IrqDisable)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("State Bit - {:?}", control_bits.state_bit)}</span>
                        <div class="input-group-text">
                            <input type="checkbox" checked={control_bits.state_bit} onclick={ctx.link().callback(|_|{Msg::UpdateFlag(UpdateFlagType::StateBit)})}/>
                        </div>
                    </div>
                </div>

                <div class="input-group m-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text cpsr-text">{&format!("Mode Bits - {:b}", control_bits.mode_bits)}</span>
                    </div>
                </div>
            </div>
            
        }
    }
}
