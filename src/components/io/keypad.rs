use yew::{html, Html};
use crate::components::io_reg::{IORegisters};

impl IORegisters {
    pub fn view_key_status(&self) -> Html {
        html! {
            <div class="io-reg-section">
                <div class="io-reg-section-header" id="key-status-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#key-status"
                                aria-expanded="true" aria-controls="key-status">
                            {"Key Status"}
                        </button>
                    </h5>
                </div>

                <div id="key-status" class="io-reg-section-body collapse" aria-labelledby="key-status-heading">
                    <div >
                        <table class="table register-table">
                            <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>{"Button A"}</td>
                                <td>{self.props.gba.borrow().key_status.get_button_a()}</td>
                            </tr>
                            <tr>
                                <td>{"Button B"}</td>
                                <td>{self.props.gba.borrow().key_status.get_button_b()}</td>
                            </tr>
                            <tr>
                                <td>{"Button Select"}</td>
                                <td>{self.props.gba.borrow().key_status.get_button_select()}</td>
                            </tr>
                            <tr>
                                <td>{"Button Start"}</td>
                                <td>{self.props.gba.borrow().key_status.get_button_start()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Right"}</td>
                                <td>{self.props.gba.borrow().key_status.get_dpad_right()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Left"}</td>
                                <td>{self.props.gba.borrow().key_status.get_dpad_left()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Up"}</td>
                                <td>{self.props.gba.borrow().key_status.get_dpad_up()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Down"}</td>
                                <td>{self.props.gba.borrow().key_status.get_dpad_down()}</td>
                            </tr>
                            <tr>
                                <td>{"Button R"}</td>
                                <td>{self.props.gba.borrow().key_status.get_button_r()}</td>
                            </tr>
                            <tr>
                                <td>{"Button L"}</td>
                                <td>{self.props.gba.borrow().key_status.get_button_l()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    pub fn view_key_interrupt_control(&self) -> Html {
        html! {
            <div class="io-reg-section">
                <div class="io-reg-section-header" id="key-interrupt-control-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#key-interrupt-control"
                                aria-expanded="true" aria-controls="key-interrupt-control">
                            {"Key Interrupt Control"}
                        </button>
                    </h5>
                </div>

                <div id="key-interrupt-control" class="io-reg-section-body collapse" aria-labelledby="key-interrupt-control-heading">
                    <div >
                        <table class="table register-table">
                            <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                            </thead>
                            <tbody>
                            <tr>
                                <td>{"Button A"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_button_a()}</td>
                            </tr>
                            <tr>
                                <td>{"Button B"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_button_b()}</td>
                            </tr>
                            <tr>
                                <td>{"Button Select"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_button_select()}</td>
                            </tr>
                            <tr>
                                <td>{"Button Start"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_button_start()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Right"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_dpad_right()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Left"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_dpad_left()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Up"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_dpad_up()}</td>
                            </tr>
                            <tr>
                                <td>{"DPad Down"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_dpad_down()}</td>
                            </tr>
                            <tr>
                                <td>{"Button R"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_button_r()}</td>
                            </tr>
                            <tr>
                                <td>{"Button L"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_button_l()}</td>
                            </tr>
                            <tr>
                                <td>{"IRQ Enable Flag"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_irq_enable_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"IRQ Condition"}</td>
                                <td>{self.props.gba.borrow().ket_interrupt_control.get_irq_condition()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}