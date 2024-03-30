use base64::{engine::general_purpose, Engine};
use rrplug::prelude::*;

use crate::{map_info::load_furnace_brush_data, FURNACE};

pub fn sever_register_sqfunction() {
    register_sq_functions(push_map_name);
    register_sq_functions(push_mesh);
    register_sq_functions(get_meshes);
    register_sq_functions(remove_mesh);
    register_sq_functions(move_mesh);
    register_sq_functions(get_last_compiled_map);
    register_sq_functions(get_furnace_data_base64);
    register_sq_functions(set_texture_for_mesh);
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "PushMapName")]
fn push_map_name(map_name: String) {
    let mut furnace = FURNACE.wait().lock().unwrap();
    furnace.current_map = map_name;
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "PushMesh")]
pub fn push_mesh(point1: Vector3, point2: Vector3) -> i32 {
    let mut furnace = FURNACE.wait().lock().unwrap();

    furnace.meshes.push(Some([point1, point2]));
    furnace.texture_map.push("$w".into());

    furnace.meshes.len().saturating_sub(1) as i32
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "GetMeshes")]
pub fn get_meshes(map: String) -> Vec<Vector3> {
    let mut furnace = FURNACE.wait().lock().unwrap();

    load_furnace_brush_data(&mut furnace, map);

    furnace
        .meshes
        .clone()
        .into_iter()
        .flatten()
        .flatten()
        .collect()
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "RemoveMesh")]
pub fn remove_mesh(index: i32) {
    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.meshes.get_mut(index as usize) {
        Some(mesh) => _ = mesh.take(),
        None => log::warn!("no mesh found"),
    }
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "MoveMesh")]
pub fn move_mesh(index: i32, dir: Vector3) -> i32 {
    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.meshes.get_mut(index as usize) {
        Some(mesh) => match mesh {
            Some(mesh) => {
                mesh[0] = mesh[0] + dir;
                mesh[1] = mesh[1] + dir;
                0
            }
            None => 1,
        },
        None => {
            log::warn!("no mesh found");
            1
        }
    }
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "SetTextureForMesh")]
pub fn set_texture_for_mesh(index: i32, new_texture: String) -> i32 {
    let mut furnace = FURNACE.wait().lock().unwrap();

    match furnace.texture_map.get_mut(index as usize) {
        Some(texture) => *texture = new_texture.into(),
        None => {
            log::warn!("no texture found");
            return 1;
        }
    }
    0
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "GetLastCompiledMap")]
pub fn get_last_compiled_map() -> String {
    let furnace = FURNACE.wait().lock().unwrap();

    furnace.last_compiled.clone()
}

// test command
// script foreach( var f in GetFurnace64BaseData("mp_default") ) { printt( f, "\n" ) }
#[rrplug::sqfunction(VM = "SERVER", ExportName = "GetFurnace64BaseData")]
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

    (1..max_slices)
        .map(|index| {
            if index == max_slices.saturating_sub(1) {
                buf[index.saturating_sub(1) * SLICE_LENGHT..].to_string()
            } else {
                buf[index.saturating_sub(1) * SLICE_LENGHT..index * SLICE_LENGHT].to_string()
            }
        })
        .collect()
}
