use yew::{html, Html};
use crate::components::io_reg::{IORegisters};

impl IORegisters {
    pub fn view_key_status(&self) -> Html {
        html! {
            <details class="io-reg-section">
<summary class="io-reg-section-header">{"Key Status"}</summary>

                        <table class="data-table">
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
                    
</details>
        }
    }

    pub fn view_key_interrupt_control(&self) -> Html {
        html! {
            <details class="io-reg-section">
<summary class="io-reg-section-header">{"Key Interrupt Control"}</summary>

                        <table class="data-table">
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
                    
</details>
        }
    }
}