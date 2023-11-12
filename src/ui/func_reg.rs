use rrplug::prelude::*;

use crate::ui::WINDOW_GLOBAL_DATA;

pub fn ui_register_sqfunction(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(get_grid);
    plugin_data.register_sq_functions(get_eye_distance);
    plugin_data.register_sq_functions(get_current_mesh);
    plugin_data.register_sq_functions(get_nudge_value);
    plugin_data.register_sq_functions(get_texture);
}

#[rrplug::sqfunction(VM = "UI", ExportName = "FurnaceGetGrid")]
fn get_grid() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    window_data.grid
}

#[rrplug::sqfunction(VM = "UI", ExportName = "FurnaceGetEyeDistance")]
fn get_eye_distance() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    window_data.eye_distance
}

#[rrplug::sqfunction(VM = "UI", ExportName = "FurnaceGetCurrentMesh")]
fn get_current_mesh() -> i32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    window_data.mesh_id.unwrap_or_default()
}

#[rrplug::sqfunction(VM = "UI", ExportName = "FurnaceGetNudgeValue")]
fn get_nudge_value() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    window_data.nudge
}

#[rrplug::sqfunction(VM = "UI", ExportName = "FurnaceGetTexture")]
fn get_texture() -> String {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    window_data.texture.clone()
}
