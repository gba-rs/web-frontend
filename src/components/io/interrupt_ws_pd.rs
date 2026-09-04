use yew::{html, Html};
use crate::components::io_reg::{IORegisters};

impl IORegisters {
    pub fn view_interrupt_master_enable_register(&self) -> Html {
        html! {
            <details class="io-reg-section">
<summary class="io-reg-section-header">{"Interrupt Master Enable Register"}</summary>

                        <table class="data-table">
                            <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>{"Disable"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ime_interrupt.get_disable()}</td>
                            </tr>
                            </tbody>
                        </table>
                    
</details>
        }
    }

    pub fn view_interrupt_enable_register(&self) -> Html {
        html! {
            <details class="io-reg-section">
<summary class="io-reg-section-header">{"Interrupt Enable Register"}</summary>

                        <table class="data-table">
                            <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>{"LCD V Blank"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_lcd_v_blank()}</td>
                            </tr>
                            <tr>
                                <td>{"LCD H Blank"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_lcd_h_blank()}</td>
                            </tr>
                            <tr>
                                <td>{"LCD V Counter"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_lcd_v_counter_()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer Zero Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_timer_zero_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer One Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_timer_one_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer Two Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_timer_two_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer Three Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_timer_three_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Serial Communication"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_serial_communication()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA Zero"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_dma_zero()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA One"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_dma_zero()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA Two"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_dma_two()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA Three"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_dma_three()}</td>
                            </tr>
                            <tr>
                                <td>{"Keypad"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_keypad()}</td>
                            </tr>
                            <tr>
                                <td>{"Game Pack"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.ie_interrupt.get_game_pack()}</td>
                            </tr>
                            </tbody>
                        </table>
                    
</details>
        }
    }

    pub fn view_interrupt_request_flags(&self) -> Html {
        html! {
            <details class="io-reg-section">
<summary class="io-reg-section-header">{"Interrupt Request Flags"}</summary>

                        <table class="data-table">
                            <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>{"LCD V Blank"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_lcd_v_blank()}</td>
                            </tr>
                            <tr>
                                <td>{"LCD H Blank"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_lcd_h_blank()}</td>
                            </tr>
                            <tr>
                                <td>{"LCD V Counter"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_lcd_v_counter()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer Zero Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_timer_zero_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer One Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_timer_one_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer Two Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_timer_two_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Timer Three Overflow"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_timer_three_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Serial Communication"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_serial_communication()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA Zero"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_dma_zero()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA One"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_dma_zero()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA Two"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_dma_two()}</td>
                            </tr>
                            <tr>
                                <td>{"DMA Three"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_dma_three()}</td>
                            </tr>
                            <tr>
                                <td>{"Keypad"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_keypad()}</td>
                            </tr>
                            <tr>
                                <td>{"Game Pack"}</td>
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_game_pack()}</td>
                            </tr>
                            </tbody>
                        </table>
                    
</details>
        }
    }

    pub fn view_wait_state(&self) -> Html {
        html! {
            <details class="io-reg-section">
<summary class="io-reg-section-header">{"Wait State"}</summary>

                        <table class="data-table">
                            <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>{"SRAM Wait Control"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_sram_wait_control()}</td>
                            </tr>
                            <tr>
                                <td>{"Wait State 0 - 1st Access"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_wait_state_zero_first_access()}</td>
                            </tr>
                            <tr>
                                <td>{"Wait State 0 - 2nd Access"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_wait_state_zero_second_access()}</td>
                            </tr>
                            <tr>
                                <td>{"Wait State 1 - 1st Access"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_wait_state_one_first_access()}</td>
                            </tr>
                            <tr>
                                <td>{"Wait State 1 - 2nd Access"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_wait_state_one_second_access()}</td>
                            </tr>
                            <tr>
                                <td>{"Wait State 2 - 1st Access"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_wait_state_two_first_access()}</td>
                            </tr>
                            <tr>
                                <td>{"Wait State 2 - 2nd Access"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_wait_state_two_second_access()}</td>
                            </tr>
                            <tr>
                                <td>{"Phi Terminal Output"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_phi_terminal_output()}</td>
                            </tr>
                            <tr>
                                <td>{"Gamepak Prefetch Buffer"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_gamepak_prefetch_buffer()}</td>
                            </tr>
                            <tr>
                                <td>{"Gamepak Type Flag"}</td>
                                <td>{self.props.gba.borrow().memory_bus.cycle_clock.wait_state_control.get_gamepak_type_flag()}</td>
                            </tr>
                            </tbody>
                        </table>
                    
</details>
        }
    }
}