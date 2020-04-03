use yew::{html, Html};
use crate::components::io_reg::{IORegisters};

pub trait LCD {
    fn view_display_control(&self) -> Html;
    fn view_display_status(&self) -> Html;
    fn view_green_swap(&self) -> Html;
    fn view_bg(&self, bg_number: usize) -> Html;
    fn view_bg_affine_component(&self, bg_number: usize) -> Html;
    fn view_window(&self, window_number: usize) -> Html;
    fn view_control_window_inside(&self) -> Html;
    fn view_control_window_outside(&self) -> Html;
    fn view_mosaic_size(&self) -> Html;
    fn view_color_spec_effect_selection(&self) -> Html;
    fn view_alpha_blending(&self) -> Html;
    fn view_brightness(&self) -> Html;
}

impl LCD for IORegisters {
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

                <div id="display-control" class="collapse" aria-labelledby="display-control-heading">
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

    fn view_green_swap(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="green-swap-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#green-swap"
                                aria-expanded="true" aria-controls="green-swap">
                            {"Green Swap"}
                        </button>
                    </h5>
                </div>

                <div id="green-swap" class="collapse" aria-labelledby="green-swap-heading">
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
                                <td>{"Green Swap"}</td>
                                <td>{self.props.gba.borrow().gpu.green_swap.get_green_swap()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_bg(&self, bg_number: usize) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id={format!("bg{}-heading", bg_number + 1)}>
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target={format!("#bg{}", bg_number + 1)}
                                aria-expanded="true" aria-controls={format!("bg{}", bg_number + 1)}>
                            {format!("Background {}", bg_number + 1)}
                        </button>
                    </h5>
                </div>

                <div id={format!("bg{}", bg_number + 1)} class="collapse" aria-labelledby={format!("bg{}-heading", bg_number + 1)}  >
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
                                <td>{"BG Priority"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_bg_priority()}</td>
                            </tr>
                            <tr>
                                <td>{"Character Base Block"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_character_base_block()}</td>
                            </tr>
                            <tr>
                                <td>{"Mosaic"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_mosaic()}</td>
                            </tr>
                            <tr>
                                <td>{"Colors"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_colors()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Base Block"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_screen_base_block()}</td>
                            </tr>
                            <tr>
                                <td>{"Display Area Overflow"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_display_area_overflow()}</td>
                            </tr>
                            <tr>
                                <td>{"Screen Size"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].control.get_screen_size()}</td>
                            </tr>
                            <tr>
                                <td>{"Horizontal Offset"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].horizontal_offset.get_offset()}</td>
                            </tr>
                            <tr>
                                <td>{"Vertical Offset"}</td>
                                <td>{self.props.gba.borrow().gpu.backgrounds[bg_number].vertical_offset.get_offset()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_bg_affine_component(&self, bg_number: usize) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id={format!("bg_affine{}-heading", bg_number + 1)}>
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target={format!("#bg_affine{}", bg_number + 1)}
                                aria-expanded="true" aria-controls={format!("bg_affine{}", bg_number + 1)}>
                            {format!("BG Affine Component {}", bg_number + 1)}
                        </button>
                    </h5>
                </div>

                <div id={format!("bg_affine{}", bg_number + 1)} class="collapse" aria-labelledby={format!("bg_affine{}-heading", bg_number + 1)}  >
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
                                <td>{"Reference Point X Internal"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_x_internal}</td>
                            </tr>
                            <tr>
                                <td>{"External X - Fractional Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_x_external.get_fractional_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"External X - Integer Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_x_external.get_integer_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"External X - Sign"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_x_external.get_sign()}</td>
                            </tr>
                            <tr>
                                <td>{"Reference Point Y Internal"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_y_internal}</td>
                            </tr>
                            <tr>
                                <td>{"External Y - Fractional Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_y_external.get_fractional_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"External Y - Integer Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_y_external.get_integer_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"External Y - Sign"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].refrence_point_y_external.get_sign()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. A - Fractional"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_a.get_fractional_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. A - Integer Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_a.get_integer_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. A - Sign"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_a.get_sign()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. B - Fractional"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_b.get_fractional_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. B - Integer Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_b.get_integer_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. B - Sign"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_b.get_sign()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. C - Fractional"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_c.get_fractional_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. C - Integer Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_c.get_integer_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. C - Sign"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_c.get_sign()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. D - Fractional"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_d.get_fractional_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. D - Integer Portion"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_d.get_integer_portion()}</td>
                            </tr>
                            <tr>
                                <td>{"Rot. D - Sign"}</td>
                                <td>{self.props.gba.borrow().gpu.bg_affine_components[bg_number].rotation_scaling_param_d.get_sign()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
    fn view_window(&self, window_number: usize) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id={format!("window{}-heading", window_number + 1)}>
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target={format!("#window{}", window_number + 1)}
                                aria-expanded="true" aria-controls={format!("window{}", window_number + 1)}>
                            {format!("Window {}", window_number + 1)}
                        </button>
                    </h5>
                </div>

                <div id={format!("window{}", window_number + 1)} class="collapse" aria-labelledby={format!("window{}-heading", window_number + 1)}  >
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
                                <td>{"X1"}</td>
                                <td>{self.props.gba.borrow().gpu.windows[window_number].horizontal_dimensions.get_X1()}</td>
                            </tr>
                            <tr>
                                <td>{"X2"}</td>
                                <td>{self.props.gba.borrow().gpu.windows[window_number].horizontal_dimensions.get_X2()}</td>
                            </tr>
                            <tr>
                                <td>{"Y1"}</td>
                                <td>{self.props.gba.borrow().gpu.windows[window_number].vertical_dimensions.get_Y1()}</td>
                            </tr>
                            <tr>
                                <td>{"Y2"}</td>
                                <td>{self.props.gba.borrow().gpu.windows[window_number].vertical_dimensions.get_Y2()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_control_window_inside(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="control-window-inside-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#control-window-inside"
                                aria-expanded="true" aria-controls="control-window-inside">
                            {"Control Window Inside"}
                        </button>
                    </h5>
                </div>

                <div id="control-window-inside" class="collapse" aria-labelledby="control-window-inside-heading">
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
                                <td>{"Window 0 BG Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_inside.get_window0_bg_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 0 Obj Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_inside.get_window0_obj_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 0 Color Special Effect"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_inside.get_window0_color_special_effect()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 1 BG Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_inside.get_window1_bg_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 1 Obj Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_inside.get_window1_obj_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Window 1 Color Special Effect"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_inside.get_window1_color_special_effect()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_control_window_outside(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="control-window-outside-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#control-window-outside"
                                aria-expanded="true" aria-controls="control-window-outside">
                            {"Control Window Outside"}
                        </button>
                    </h5>
                </div>

                <div id="control-window-outside" class="collapse" aria-labelledby="control-window-outside-heading">
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
                                <td>{"BG Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_outside.get_outside_bg_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_outside.get_outside_obj_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Color Effects"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_outside.get_outside_color_special_effect()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Window BG Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_outside.get_obj_window_bg_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Window Obj Enable Bits"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_outside.get_obj_window_obj_enable_bits()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Window Color Special Effect"}</td>
                                <td>{self.props.gba.borrow().gpu.control_window_outside.get_obj_window_color_special_effect()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_mosaic_size(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="mosaic-size-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#mosaic-size"
                                aria-expanded="true" aria-controls="mosaic-size">
                            {"Mosaic Size"}
                        </button>
                    </h5>
                </div>

                <div id="mosaic-size" class="collapse" aria-labelledby="mosaic-size-heading">
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
                                <td>{"BG Mosaic HSize"}</td>
                                <td>{self.props.gba.borrow().gpu.mosaic_size.get_bg_mosaic_hsize()}</td>
                            </tr>
                            <tr>
                                <td>{"BG Mosaic VSize"}</td>
                                <td>{self.props.gba.borrow().gpu.mosaic_size.get_bg_mosaic_vsize()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Mosaic HSize"}</td>
                                <td>{self.props.gba.borrow().gpu.mosaic_size.get_obj_mosaic_hsize()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj Mosaic VSize"}</td>
                                <td>{self.props.gba.borrow().gpu.mosaic_size.get_obj_mosaic_vsize()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_color_spec_effect_selection(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="color-spec-effec-sel-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#color-spec-effec-sel"
                                aria-expanded="true" aria-controls="color-spec-effec-sel">
                            {"Color Special Effect Selection"}
                        </button>
                    </h5>
                </div>

                <div id="color-spec-effec-sel" class="collapse" aria-labelledby="color-spec-effec-sel-heading">
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
                                <td>{"BG0 1st Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg0_1st_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BG1 1st Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg1_1st_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BG2 1st Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg2_1st_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BG3 1st Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg3_1st_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj 1st Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_obj_1st_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BD 1st Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bd_1st_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"Color Special Effect"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_color_special_effect()}</td>
                            </tr>
                            <tr>
                                <td>{"BG0 2nd Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg0_2nd_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BG1 2nd Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg1_2nd_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BG2 2nd Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg2_2nd_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BG3 2nd Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bg3_2nd_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"Obj 2nd Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_obj_2nd_target_pixel()}</td>
                            </tr>
                            <tr>
                                <td>{"BD 2nd Target Pixel"}</td>
                                <td>{self.props.gba.borrow().gpu.color_special_effects_selection.get_bd_2nd_target_pixel()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_alpha_blending(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="alpha-blending-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#alpha-blending"
                                aria-expanded="true" aria-controls="alpha-blending">
                            {"Alpha Blending Coefficients"}
                        </button>
                    </h5>
                </div>

                <div id="alpha-blending" class="collapse" aria-labelledby="alpha-blending-heading">
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
                                <td>{"EVA Coefficient"}</td>
                                <td>{self.props.gba.borrow().gpu.alpha_blending_coefficients.get_eva_coefficient()}</td>
                            </tr>
                            <tr>
                                <td>{"EVB Coefficient"}</td>
                                <td>{self.props.gba.borrow().gpu.alpha_blending_coefficients.get_evb_coefficient()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }

    fn view_brightness(&self) -> Html {
        html! {
            <div class="card border-0">
                <div class="card-header p-0 bg-transparent" id="brightness-heading">
                    <h5 class="mb-0">
                        <button class="btn btn-link text-dark" data-toggle="collapse" data-target="#brightness"
                                aria-expanded="true" aria-controls="brightness">
                            {"Brightness Coefficient"}
                        </button>
                    </h5>
                </div>

                <div id="brightness" class="collapse" aria-labelledby="brightness-heading">
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
                                <td>{"EVY Coefficient"}</td>
                                <td>{self.props.gba.borrow().gpu.brightness_coefficient.get_evy_coefficient()}</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}