use yew::{html, Html};
use crate::components::io_reg::IORegisters;

impl IORegisters {
    pub fn view_lcd(&self) -> Html {
        html! {
            <div id="lcd-accordion">
                {self.view_display_control()}
                {self.view_display_status()}
            </div>
        }
    }

    fn view_display_control(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="display-control-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#display-control"
                                aria-expanded="true" aria-controls="display-control">
                            {"Display Control"}
                        </button>
                    </h5>
                </div>

                <div id="display-control" class="collapse show" aria-labelledby="display-control-heading">
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
                                <td>{"BG Mode"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_bg_mode()}</td>
                            </tr>
                            <tr>
                                <td>{"CGB Mode"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_cgb_mode()}</td>
                            </tr>
                            <tr>
                                <td>{"Display Frame Select"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_display_frame_select()}</td>
                            </tr>
                            <tr>
                                <td>{"Hblank Interval Free"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_hblank_interval_free()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Character VRAM Mapping"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_obj_charcter_vram_mapping()}</td>
                            </tr>
                            <tr>
                                <td>{"Forced Blank"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_forced_blank()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG0"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg0()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG1"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg1()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG2"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg2()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display BG3"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_bg3()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Display Obj"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_screen_display_obj()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 0 Display Flag"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_window_0_display_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 1 Display Flag"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_window_1_display_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"Object Window Display Flag"}</td>
                                <td>{self.props.gba.borrow().gpu.display_control.get_obj_window_display_flag()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
    fn view_display_status(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="display-status-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#display-status"
                                aria-expanded="true" aria-controls="display-status">
                            {"Display Status"}
                        </button>
                    </h5>
                </div>

                <div id="display-status" class="collapse" aria-labelledby="display-status-heading">
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
                                <td>{"VBlank Flag"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_vblank_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"HBlank Flag"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_hblank_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"VCounter Flag"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_vcounter_flag()}</td>
                            </tr>
                            <tr>
                                <td>{"VBlank IRQ Enable"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_vblank_irq_enable()}</td>
                            </tr>
                            <tr>
                                <td>{"HBlank IRQ Enable"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_hblank_irq_enable()}</td>
                            </tr>
                            <tr>
                                <td>{"VCounter IRQ Enable"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_vcounter_irq_enable()}</td>
                            </tr>
                            <tr>
                                <td>{"VCount Setting"}</td>
                                <td>{self.props.gba.borrow().gpu.display_status.get_vcount_setting()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>

        }
    }
}
