use base64::{engine::general_purpose, Engine};
use rrplug::{
    bindings::{squirreldatatypes::HSquirrelVM, unwraped::SquirrelFunctionsUnwraped},
    prelude::*,
    sq_return_int, sq_return_notnull, sq_return_null, sq_return_string, sqfunction,
    wrappers::squirrel::{push_sq_array, push_sq_string, push_sq_vector},
};

use crate::{map_info::load_furnace_brush_data, FURNACE};

pub fn sever_register_sqfunction(plugin_data: &PluginData) {
    _ = plugin_data.register_sq_functions(info_push_map_name);
    _ = plugin_data.register_sq_functions(info_push_mesh);
    _ = plugin_data.register_sq_functions(info_get_meshes);
    _ = plugin_data.register_sq_functions(info_remove_mesh);
    _ = plugin_data.register_sq_functions(info_get_last_compiled_map);
    _ = plugin_data.register_sq_functions(info_get_furnace_data_base64);
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

#[sqfunction(VM=SERVER,ExportName=GetMeshes)]
pub fn get_meshes(map: String) -> Vec<Vector3> {
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

// test command 
// script foreach( var f in GetFurnace64BaseData("mp_default") ) { printt( f, "\n" ) }
#[sqfunction(VM=SERVER,ExportName=GetFurnace64BaseData)]
pub fn get_furnace_data_base64(map: String) -> Vec<String> {
    const SLICE_LENGHT: usize = 100;

    log::info!("called get_furnace_data_base64");

    let furnace = FURNACE.wait().lock().unwrap();

    let path = furnace.path.join(format!("Titanfall2/maps/{map}.furnace"));

    let raw = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            log::error!("failed to load furnace data: {err}");
            String::new()
        }
    };

    let mut buf = String::new();
    general_purpose::STANDARD_NO_PAD.encode_string(raw.as_bytes(), &mut buf);

    let max_slices = buf.len().div_ceil(SLICE_LENGHT);

    let furnace_slices: Vec<String> = (1..max_slices)
        .map(|index| {
            if index == max_slices.saturating_sub(1) {
                buf[index.saturating_sub(1)*SLICE_LENGHT..].to_string()
            } else {
                buf[index.saturating_sub(1)*SLICE_LENGHT..index*SLICE_LENGHT].to_string()
            }
        })
        .collect();

    let push_closures = furnace_slices.into_iter()
        .map(|slice| {
            move |sqvm: *mut HSquirrelVM, sqfunctions: &SquirrelFunctionsUnwraped| {
                push_sq_string(sqvm, sqfunctions, slice)
            }
        })
        .collect();

    push_sq_array(sqvm, sq_functions, push_closures);

    sq_return_notnull!()
}