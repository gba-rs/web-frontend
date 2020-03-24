use yew::{html, Html};
use crate::components::io_reg::IORegisters;

impl IORegisters {
    pub fn view_lcd(&self) -> Html {
        html! {
            <div>
                <h6>{"Display Control"}</h6>
                    <table class="table register-table">
                        <thead>
                            <tr>
                                <th scope="col">{"Field"}</th>
                                <th scope="col">{"Val Hex"}</th>
                                <th scope="col">{"Val Dec"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"BG Mode"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_bg_mode())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_bg_mode()}</td>
                            </tr>
                            <tr>
                                <td>{"CGB Mode"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_cbg_mode())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_cbg_mode()}</td>
                            </tr>
                            <tr>
                                <td>{"Display From Select"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_cbg_mode())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_cbg_mode()}</td>
                            </tr>
                            <tr>
                                <td>{"Display From Select"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_display_from_select())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_display_from_select()}</td>
                            </tr>
                        </tbody>
                    </table>
            </div>
        }
    }
}
