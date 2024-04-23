#![allow(clippy::format_collect)]
use once_cell::sync::Lazy;
use rrplug::prelude::*;
use std::{collections::HashMap, fs, sync::Arc};

use crate::FurnaceData;

const BASE: &str = include_str!("../base.map");
const BRUSH_START: u32 = 7;
pub static TEXTURE_MAP: Lazy<HashMap<&'static str, Arc<str>>> = Lazy::new(|| {
    let mut texture_hash = HashMap::new();

    _ = texture_hash.insert("$w", "world/dev/dev_white_512".to_string().into());
    _ = texture_hash.insert("$g", "world/dev/dev_ground_512".to_string().into());
    _ = texture_hash.insert("$r", "world/dev/dev_red_512".to_string().into());
    _ = texture_hash.insert("$b", "world/dev/dev_blue_512".to_string().into());
    _ = texture_hash.insert("$b", "world/dev/dev_blue_512".to_string().into());
    _ = texture_hash.insert("$win", "world/windows".to_string().into());
    _ = texture_hash.insert("$wood", "world/wood".to_string().into());
    _ = texture_hash.insert("$c", "world/concrete".to_string().into());
    _ = texture_hash.insert("$brick", "world/brick".to_string().into());
    _ = texture_hash.insert("$nodraw", "tools/toolsnodraw".to_string().into());
    _ = texture_hash.insert("$window", "tools/toolswindowhint".to_string().into());

    texture_hash
});

#[derive(Default)]
pub struct FurnaceFileData {
    pub meshes: Vec<Option<[Vector3; 2]>>,
    pub texture_map: Vec<Arc<str>>,
}

pub fn write_map_file(furnace: &FurnaceData, map_file: String) {
    if furnace.brushes.is_empty() {
        return;
    }

    log::info!("creating new map file");

    let base = BASE.to_string();

    let body: String = furnace
        .brushes
        .iter()
        .enumerate()
        .map(|mesh| format!("// brush {}\n{}\n", BRUSH_START as usize + mesh.0, mesh.1))
        .collect();

    let map_txt = base.replace("/*8*/", &body);

    let path = furnace.path.join(format!("Titanfall2/maps/{map_file}"));

    match fs::write(path, map_txt) {
        Ok(_) => log::info!("created new map file"),
        Err(err) => log::error!("failed to creat new map file: {err}"),
    }
}

pub fn write_furnace_brush_data(furnace: &FurnaceData, map: String) {
    if furnace.meshes.is_empty() {
        return;
    }

    log::info!("creating new furnace file");

    let mesh_data: String = furnace
        .meshes
        .iter()
        .filter(|m| m.is_some())
        .map(|m| {
            let point1 = m.unwrap()[0];
            let point2 = m.unwrap()[1];
            format!(
                "({},{},{});({},{},{})\n",
                point1.x.round(),
                point1.y.round(),
                point1.z.round(),
                point2.x.round(),
                point2.y.round(),
                point2.z.round()
            )
        })
        .collect();

    let texture_data: String = furnace
        .texture_map
        .iter()
        .map(|t| format!("{t}\n"))
        .collect();

    let furnace_format = format!(
        "###Mesh##\n{}\n###Textures##\n{}",
        mesh_data.trim_end().trim(),
        texture_data.trim_end().trim()
    );

    let path = furnace.path.join(format!("Titanfall2/maps/{map}.furnace"));

    match fs::write(path, furnace_format) {
        Ok(_) => log::info!("created new furnace file"),
        Err(err) => log::error!("failed to create new furnace file: {err}"),
    }
}

pub fn load_furnace_brush_data(furnace: &mut FurnaceData, map: String) {
    log::info!("loadingfurnace file");

    let path = furnace.path.join(format!("Titanfall2/maps/{map}.furnace"));

    let data = match fs::read_to_string(path) {
        Ok(s) => parse_furnace_data(s),
        Err(err) => {
            log::error!("failed to load furnace data: {err}");
            FurnaceFileData::default()
        }
    };

    furnace.meshes = data.meshes;
    furnace.texture_map = data.texture_map;
}

pub fn parse_furnace_data(data: String) -> FurnaceFileData {
    let info_chunks: HashMap<String, String> = HashMap::from_iter(data.split("###").map(|chunk| {
        let mut split = chunk.split("##");

        let name = split.next().unwrap_or("ohno").to_owned();
        let value = split.next().unwrap_or("ohno").trim_end().trim().to_owned();

        (name, value)
    }));

    let meshes: Vec<Option<[Vector3; 2]>> = match info_chunks.get("Mesh") {
        Some(data) => data
            .split('\n')
            .map(|line| {
                let points: Vec<Vector3> = line
                    .split(';')
                    .map(|point| {
                        let p = point
                            .split(',')
                            .map(|cord| {
                                let cord = cord.strip_suffix(')').unwrap_or(cord);
                                let cord = cord.strip_prefix('(').unwrap_or(cord);
                                cord.into()
                            })
                            .collect::<Vec<String>>();

                        Vector3::from([
                            p[0].parse().unwrap(),
                            p[1].parse().unwrap(),
                            p[2].parse().unwrap(),
                        ])
                    })
                    .collect();

                Some([points[0], points[1]])
            })
            .collect(),
        None => Vec::new(),
    };

    // so considering that we would have to allocate a strings for each mesh and brush
    // arc might be a good idea XD

    let textures: Vec<String> = match info_chunks.get("Textures") {
        Some(data) => data.split('\n').map(|s| s.to_owned()).collect(),
        None => Vec::new(),
    };

    FurnaceFileData {
        meshes,
        texture_map: textures.into_iter().map(|s| s.into()).collect(), // could be optimized later to remove the same textures
    }
}

pub fn get_path_texture(texture: &Arc<str>) -> Arc<str> {
    return match TEXTURE_MAP.get(&texture[..]) {
        Some(t) => t.clone(),
        None => texture.clone(),
    };
}
