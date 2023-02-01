use base64::{engine::general_purpose, Engine};
use rrplug::{
    prelude::*, sq_raise_error, sq_return_null, sqfunction, wrappers::northstar::ScriptVmType,
};

use crate::{compile::compile_map, map_info::parse_furnace_data, FURNACE};

pub fn client_register_sqfunction(plugin_data: &PluginData) {
    _ = plugin_data.register_sq_functions(info_push_map_name_cl);
    _ = plugin_data.register_sq_functions(info_compile_map_from_raw_data);
}

#[sqfunction(VM=CLIENT,ExportName=ClientPushMapName)]
fn push_map_name_cl(map_name: String) {
    log::info!("called push_map_name_cl");

    let mut furnace = FURNACE.wait().lock().unwrap();
    furnace.current_map = map_name;

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

        furnace.meshes = parse_furnace_data(decoded);
    }

    compile_map(ScriptVmType::Ui);

    sq_return_null!()
}
