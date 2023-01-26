use rrplug::log;
use std::fs;

use crate::FurnaceData;

const BASE: &str = include_str!("../base.map");
const BRUSH_START: u32 = 7;

pub fn write_map_file(furnace: &mut FurnaceData, map_file: String) {
    if furnace.meshes.is_empty() {
        return;
    }

    log::info!("creating new map file");

    let base = BASE.to_string();

    let body: String = furnace
        .meshes
        .iter()
        .enumerate()
        .map(|mesh| format!("// brush {}\n{}\n", BRUSH_START as usize + mesh.0, mesh.1))
        .collect();

    println!("{body}");

    furnace.meshes.clear();

    let map_txt = base.replace("/*8*/", &body);

    let path = furnace.path.join(format!("Titanfall2/maps/{map_file}"));

    match fs::write(path, map_txt) {
        Ok(_) => log::info!("created new map file"),
        Err(err) => log::error!("failed to creat new map file: {err}"),
    }
}
