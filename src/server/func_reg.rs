use base64::{engine::general_purpose, Engine};
use rrplug::{
    prelude::*,
    sq_return_int, sq_return_notnull, sq_return_null, sq_return_string, sqfunction,
    wrappers::squirrel::push_sq_array,
};

use crate::{map_info::load_furnace_brush_data, FURNACE};

pub fn sever_register_sqfunction(plugin_data: &PluginData) {
    plugin_data.register_sq_functions(info_push_map_name);
    plugin_data.register_sq_functions(info_push_mesh);
    plugin_data.register_sq_functions(info_get_meshes);
    plugin_data.register_sq_functions(info_remove_mesh);
    plugin_data.register_sq_functions(info_move_mesh);
    plugin_data.register_sq_functions(info_get_last_compiled_map);
    plugin_data.register_sq_functions(info_get_furnace_data_base64);
    plugin_data.register_sq_functions(info_set_texture_for_mesh);
}

#[sqfunction(VM=SERVER,ExportName=PushMapName)]
fn push_map_name(map_name: String) {
    let mut furnace = FURNACE.wait().lock().unwrap();
    furnace.current_map = map_name;

    sq_return_null!()
}

#[sqfunction(VM=SERVER,ExportName=PushMesh)]
pub fn push_mesh(point1: Vector3, point2: Vector3) -> i32 {
    let mut furnace = FURNACE.wait().lock().unwrap();

    furnace.meshes.push(Some([point1, point2]));
    furnace.texture_map.push("$w".into());

    sq_return_int!(
        furnace.meshes.len().saturating_sub(1) as i32,
        sqvm,
        sq_functions
    );
}

#[sqfunction(VM=SERVER,ExportName=GetMeshes)]
pub fn get_meshes(map: String) -> Vec<Vector3> {
    let mut furnace = FURNACE.wait().lock().unwrap();

    load_furnace_brush_data(&mut furnace, map);

    let push_array = furnace
        .meshes
        .clone()
        .into_iter()
        .flatten()
        .flatten()
        .collect();

    push_sq_array(sqvm, sq_functions, push_array);

    sq_return_notnull!()
}

#[sqfunction(VM=SERVER,ExportName=RemoveMesh)]
pub fn remove_mesh(index: i32) {
    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.meshes.get_mut(index as usize) {
        Some(mesh) => _ = mesh.take(),
        None => log::warn!("no mesh found"),
    }

    sq_return_null!()
}

#[sqfunction(VM=SERVER,ExportName=MoveMesh)]
pub fn move_mesh(index: i32, dir: Vector3) -> i32 {
    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.meshes.get_mut(index as usize) {
        Some(mesh) => match mesh {
            Some(mesh) => {
                mesh[0] = mesh[0] + dir;
                mesh[1] = mesh[1] + dir;
            }
            None => {
                sq_return_int!(1, sqvm, sq_functions);
            }
        },
        None => {
            log::warn!("no mesh found");
            sq_return_int!(1, sqvm, sq_functions);
        }
    }

    sq_return_int!(0, sqvm, sq_functions);
}

#[sqfunction(VM=SERVER,ExportName=SetTextureForMesh)]
pub fn set_texture_for_mesh(index: i32, new_texture: String) -> i32 {
    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.texture_map.get_mut(index as usize) {
        Some(texture) => *texture = new_texture.into(),
        None => {
            log::warn!("no texture found");
            sq_return_int!(1, sqvm, sq_functions);
        }
    }

    sq_return_int!(0, sqvm, sq_functions);
}

#[sqfunction(VM=SERVER,ExportName=GetLastCompiledMap)]
pub fn get_last_compiled_map() -> String {
    let furnace = FURNACE.wait().lock().unwrap();

    sq_return_string!(furnace.last_compiled.clone(), sqvm, sq_functions);
}

// test command
// script foreach( var f in GetFurnace64BaseData("mp_default") ) { printt( f, "\n" ) }
#[sqfunction(VM=SERVER,ExportName=GetFurnace64BaseData)]
pub fn get_furnace_data_base64(map: String) -> Vec<String> {
    const SLICE_LENGHT: usize = 100;

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

    log::info!("max_slices : {max_slices}");
    log::info!("todo debug when there are less than 100 chars of base64");

    let furnace_slices: Vec<String> = (1..max_slices)
        .map(|index| {
            if index == max_slices.saturating_sub(1) {
                buf[index.saturating_sub(1) * SLICE_LENGHT..].to_string()
            } else {
                buf[index.saturating_sub(1) * SLICE_LENGHT..index * SLICE_LENGHT].to_string()
            }
        })
        .collect();

        push_sq_array(sqvm, sq_functions, furnace_slices);

    sq_return_notnull!()
}
