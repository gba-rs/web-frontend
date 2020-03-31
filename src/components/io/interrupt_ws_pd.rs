use yew::{html, Html};
use crate::components::io_reg::{IORegisters};

pub trait InterruptWaitStatePowerDown {
    fn view_interrupt_master_enable_register(&self) -> Html;
    fn view_interrupt_enable_register(&self) -> Html;
    fn view_interrupt_request_flags(&self) -> Html;
}

impl InterruptWaitStatePowerDown for IORegisters {
    fn view_interrupt_master_enable_register(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="interrupt-master-enable-register-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#interrupt-master-enable-register"
                                aria-expanded="true" aria-controls="interrupt-master-enable-register">
                            {"Interrupt Master Enable Register"}
                        </button>
                    </h5>
                </div>

                <div id="interrupt-master-enable-register" class="collapse" aria-labelledby="interrupt-master-enable-register-heading">
                    <div class="card-body">
                        <table class="table register-table">
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
                    </div>
                </div>
            </div>
        }
    }

    fn view_interrupt_enable_register(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="interrupt-enable-register-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#interrupt-enable-register"
                                aria-expanded="true" aria-controls="interrupt-enable-register-register">
                            {"Interrupt Enable Register"}
                        </button>
                    </h5>
                </div>

                <div id="interrupt-enable-register" class="collapse" aria-labelledby="interrupt-enable-register-heading">
                    <div class="card-body">
                        <table class="table register-table">
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
                    </div>
                </div>
            </div>
        }
    }

    fn view_interrupt_request_flags(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="interrupt-request-flags-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#interrupt-request-flags"
                                aria-expanded="true" aria-controls="interrupt-request-flags">
                            {"Interrupt Request Flags"}
                        </button>
                    </h5>
                </div>

                <div id="interrupt-request-flags" class="collapse" aria-labelledby="interrupt-request-flags-heading">
                    <div class="card-body">
                        <table class="table register-table">
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
                                <td>{self.props.gba.borrow().interrupt_handler.if_interrupt.get_lcd_v_counter_()}</td>
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
                    </div>
                </div>
            </div>
        }
    }
}