use rrplug::{
    bindings::{squirreldatatypes::HSquirrelVM, unwraped::SquirrelFunctionsUnwraped},
    prelude::*,
    sq_return_int, sq_return_notnull, sq_return_null, sq_return_string, sqfunction,
    wrappers::squirrel::{push_sq_array, push_sq_vector},
};

use crate::{map_info::load_furnace_brush_data, FURNACE};

pub fn sever_register_sqfunction(plugin_data: &PluginData) {
    _ = plugin_data.register_sq_functions(info_push_map_name);
    _ = plugin_data.register_sq_functions(info_push_mesh);
    _ = plugin_data.register_sq_functions(info_get_meshes);
    _ = plugin_data.register_sq_functions(info_remove_mesh);
    _ = plugin_data.register_sq_functions(info_get_last_compiled_map);
    _ = plugin_data.register_sq_functions(info_get_raw_furnace_data);
}

#[sqfunction(VM=SERVER,ExportName=PushMapName)]
fn push_map_name(map_name: String) {
    log::info!("called push_map_name");

    let mut furnace = FURNACE.wait().lock().unwrap();
    furnace.current_map = map_name;

    sq_return_null!()
}

#[sqfunction(VM=SERVER,ExportName=PushMesh)]
pub fn push_mesh(point1: Vector3, point2: Vector3) -> i32 {
    log::info!("called push_mesh");

    let mut furnace = FURNACE.wait().lock().unwrap();

    furnace.meshes.push(Some([point1, point2]));

    sq_return_int!(
        furnace.meshes.len().saturating_sub(1) as i32,
        sqvm,
        sq_functions
    );
}

#[sqfunction(VM=SERVER,ExportName=GetMeshes,ReturnOverwrite=array)]
pub fn get_meshes(map: String) -> Vector {
    log::info!("called get_meshes");

    let mut furnace = FURNACE.wait().lock().unwrap();

    load_furnace_brush_data(&mut furnace, map);

    let push_closures = furnace
        .meshes
        .clone()
        .into_iter()
        .flatten()
        .flatten()
        .map(|vector| {
            move |sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped| {
                push_sq_vector(sqvm, sqfunctions, vector)
            }
        })
        .collect();

    push_sq_array(sqvm, sq_functions, push_closures);

    sq_return_notnull!()
}

#[sqfunction(VM=SERVER,ExportName=RemoveMesh)]
pub fn remove_mesh(index: i32) {
    log::info!("called push_mesh");

    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.meshes.get_mut(index as usize) {
        Some(mesh) => _ = mesh.take(),
        None => log::warn!("no mesh found"),
    }

    sq_return_null!()
}

#[sqfunction(VM=SERVER,ExportName=GetLastCompiledMap)]
pub fn get_last_compiled_map() -> String {
    log::info!("called push_mesh");

    let furnace = FURNACE.wait().lock().unwrap();

    sq_return_string!(furnace.last_compiled.clone(), sqvm, sq_functions);
}

#[sqfunction(VM=SERVER,ExportName=GetRawFunaceData)]
pub fn get_raw_furnace_data(map: String) -> String {
    log::info!("called get_raw_furnace_data");

    let furnace = FURNACE.wait().lock().unwrap();

    let path = furnace.path.join(format!("Titanfall2/maps/{map}.furnace"));

    let raw = match std::fs::read_to_string(path) {
        Ok(s) => s.replace('\n', ".n."),
        Err(err) => {
            log::error!("failed to load furnace data: {err}");
            String::new()
        }
    };

    sq_return_string!(raw, sqvm, sq_functions);
}
