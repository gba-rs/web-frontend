use yew::{html, Html};
use crate::components::io_reg::IORegisters;

impl IORegisters {
    pub fn view_lcd(&self) -> Html {
        html! {
            <div>
                <h4>{"Display Control"}</h4>
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
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_cgb_mode())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_cgb_mode()}</td>
                            </tr>
                            <tr>
                                <td>{"Display Frame Select"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_display_frame_select())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_display_frame_select()}</td>
                            </tr>
                            <tr>
                                <td>{"Hblank Interval Free"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_hblank_interval_free())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_hblank_interval_free()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Character VRAM Mapping"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_obj_charcter_vram_mapping())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_obj_charcter_vram_mapping()}</td>
                            </tr>
                            <tr>
                                <td>{"Forced Blank"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_forced_blank())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_forced_blank()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG0"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_screen_display_bg0())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg0()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG1"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_screen_display_bg1())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg1()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG2"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_screen_display_bg2())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg2()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG3"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_screen_display_bg3())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg3()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display Obj"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_screen_display_obj())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_obj()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 0 Display Flag"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_window_0_display_flag())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_window_0_display_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 1 Display Flag"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_window_1_display_flag())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_window_1_display_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"Object Window Display Flag"}</td>
                                <td>{format!("{:08X}", self.props.gba.borrow().gpu.display_control.get_obj_window_display_flag())}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_obj_window_display_flag()}</td>
                            </tr>
                        </tbody>
                    </table>
            </div>
        }
    }
}
