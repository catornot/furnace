use rrplug::{prelude::*, sq_return_float, sqfunction, sq_return_int, sq_return_string};

use crate::ui::WINDOW_GLOBAL_DATA;

pub fn ui_register_sqfunction(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(info_get_grid).unwrap();
    plugin_data.register_sq_functions(info_get_eye_distance).unwrap();
    plugin_data.register_sq_functions(info_get_current_mesh).unwrap();
    plugin_data.register_sq_functions(info_get_nudge_value).unwrap();
    plugin_data.register_sq_functions(info_get_texture).unwrap();
}

#[sqfunction(VM=UI,ExportName=FurnaceGetGrid)]
fn get_grid() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    sq_return_float!(window_data.grid, sqvm, sq_functions);
}

#[sqfunction(VM=UI,ExportName=FurnaceGetEyeDistance)]
fn get_eye_distance() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    sq_return_float!(window_data.eye_distance, sqvm, sq_functions);
}

#[sqfunction(VM=UI,ExportName=FurnaceGetCurrentMesh)]
fn get_current_mesh() -> i32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    sq_return_int!(window_data.mesh_id.unwrap_or_default(), sqvm, sq_functions);
}

#[sqfunction(VM=UI,ExportName=FurnaceGetNudgeValue)]
fn get_nudge_value() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    sq_return_float!(window_data.nudge, sqvm, sq_functions);
}

#[sqfunction(VM=UI,ExportName=FurnaceGetTexture)]
fn get_texture() -> String {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    sq_return_string!(window_data.texture.clone(), sqvm, sq_functions);
}