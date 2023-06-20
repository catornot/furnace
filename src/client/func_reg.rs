use base64::{engine::general_purpose, Engine};
use rrplug::{
    prelude::*, sq_raise_error, sq_return_null, sqfunction, wrappers::northstar::ScriptVmType,
};

use crate::{compile::compile_map, map_info::parse_furnace_data, FURNACE, ui::WINDOW_GLOBAL_DATA};

pub fn client_register_sqfunction(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(info_push_map_name_cl);
    plugin_data.register_sq_functions(info_compile_map_from_raw_data);
    plugin_data.register_sq_functions(info_push_mesh_index);
    plugin_data.register_sq_functions(info_push_remove_mesh_index);
}

#[sqfunction(VM=CLIENT,ExportName=ClientPushMapName)]
fn push_map_name_cl(map_name: String) {
    let mut furnace = FURNACE.wait().lock().unwrap();
    furnace.current_map = map_name;

    sq_return_null!()
}

#[sqfunction(VM=CLIENT,ExportName=ClientPushMeshIndex)]
fn push_mesh_index(mesh_id: i32) {
    let mut window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();
    window_data.mesh_id = Some(mesh_id);

    sq_return_null!()
}

#[sqfunction(VM=CLIENT,ExportName=ClientRemoveMeshIndex)]
fn push_remove_mesh_index() {
    let mut window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();
    window_data.mesh_id = None;

    sq_return_null!()
}

#[sqfunction(VM=CLIENT,ExportName=CompileMapFromRaw)]
pub fn compile_map_from_raw_data(raw_data: String) {
    {
        let mut furnace = FURNACE.wait().lock().unwrap();

        let byte_data = raw_data.as_bytes();

        let mut buf = Vec::new();
        if let Err(err) = general_purpose::STANDARD_NO_PAD.decode_vec(byte_data, &mut buf) {
            log::error!("failed to parse base64 data {err}");
            sq_raise_error!(
                format!("failed to parse base64 data {err}"),
                sqvm,
                sq_functions
            );
        }

        let decoded = String::from_utf8_lossy(&buf).to_string();

        log::info!("{decoded}");

        let data = parse_furnace_data(decoded);
        furnace.meshes = data.meshes;
        furnace.texture_map = data.texture_map;
    }

    compile_map(ScriptVmType::Ui);

    sq_return_null!()
}