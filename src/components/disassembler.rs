use crate::app::App;
use yew::{html, Html};
use gba_emulator::{
    cpu::{
        cpu::InstructionSet,
        cpu::ARM_PC,
        cpu::THUMB_PC
    },
};

pub struct DisassemblyElement {
    address: u32,
    instruction_hex: u32,
    instruction_asm: String,
}

pub const FOLLOW_MIN: i64 = -10;
pub const FOLLOW_MAX: i64 = 10;

impl App {
    pub fn view_disassembly(&self) -> Html {
        let instruction_set = self.gba.borrow().cpu.get_instruction_set();
        let current_pc_num = if instruction_set == InstructionSet::Arm { ARM_PC } else { THUMB_PC };

        if self.disassembled {
            let current_pc = self.gba.borrow().cpu.get_register(current_pc_num);

            html! {
                <div class="code-block">
                    {for (0..self.disassembly.len()).map(|val|{
                        let element = &self.disassembly[val as usize];

                        html! {
                            <div class={if self.disassembly[val as usize].address == current_pc {"disassembly-selected"} else {""}}>
                                <span class="disassembly-address">{format!("{:08X}", element.address)}</span>
                                <span class="disassembly-hex">{format!("{:08X}", element.instruction_hex)}</span>
                                <span class="disassembly-asm">{format!("{}", element.instruction_asm)}</span>
                            </div>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div class="code-block">{"Run Disassembly"}</div>
            }
        }
    }

    pub fn follow_pc_disassemble(&mut self) {
        // Update the disassembly with the given pc follow range
        self.disassembly.clear();
        let current_pc = if self.gba.borrow().cpu.get_instruction_set() == InstructionSet::Arm { self.gba.borrow().cpu.get_register(ARM_PC) } else { self.gba.borrow().cpu.get_register(THUMB_PC) };
        let current_instruction_size = if self.gba.borrow().cpu.get_instruction_set() == InstructionSet::Arm { 4 } else { 2 };

        let mut address = current_pc as i64 + (FOLLOW_MIN * current_instruction_size);
        let total_bytes = (FOLLOW_MAX * current_instruction_size - FOLLOW_MIN * current_instruction_size) as u32;

        if address < 0 {
            address = 0;
        }

        self.disassemble(address as u32, total_bytes);
    }

    pub fn disassemble(&mut self, address: u32, total_bytes: u32) {
        let memory_block = self.gba.borrow().memory_bus.mem_map.read_block(address as u32, total_bytes);
        match self.gba.borrow().cpu.get_instruction_set() {
            InstructionSet::Arm => {
                for i in (0..memory_block.len()).step_by(4) {
                    let instruction: u32 = memory_block[i] as u32 |
                        ((memory_block[i as usize + 1] as u32) << 8) |
                        ((memory_block[i as usize + 2] as u32) << 16) |
                        ((memory_block[i as usize + 3] as u32) << 24);

                    let decode_result = self.gba.borrow().cpu.decode(instruction);
                    match decode_result {
                        Ok(decoded_instruction) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction,
                                instruction_asm: decoded_instruction.asm(),
                            });
                        }
                        Err(_) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction,
                                instruction_asm: "???".to_string(),
                            });
                        }
                    }
                }

                self.disassembled = true;
            }
            InstructionSet::Thumb => {
                for i in (0..memory_block.len()).step_by(2) {
                    let instruction: u16 = memory_block[i] as u16 | ((memory_block[i as usize + 1] as u16) << 8);

                    let decode_result = self.gba.borrow().cpu.decode(instruction as u32);
                    match decode_result {
                        Ok(decoded_instruction) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction as u32,
                                instruction_asm: decoded_instruction.asm(),
                            });
                        }
                        Err(_) => {
                            self.disassembly.push(DisassemblyElement {
                                address: (i as u32) + address,
                                instruction_hex: instruction as u32,
                                instruction_asm: "???".to_string(),
                            });
                        }
                    }
                }

                self.disassembled = true;
            }
        }
    }
}