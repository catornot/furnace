use rrplug::{log, wrappers::vector::Vector3};
use std::fs;

use crate::FurnaceData;

const BASE: &str = include_str!("../base.map");
const BRUSH_START: u32 = 7;

pub fn write_map_file(furnace: &mut FurnaceData, map_file: String) {
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

pub fn write_furnace_brush_data(furnace: &mut FurnaceData, map: String) {
    if furnace.meshes.is_empty() {
        return;
    }

    log::info!("creating new furnace file");

    furnace.meshes.sort_by(|a, b| a.1.cmp(&b.1));
    let mesh_data: String = furnace
        .meshes
        .iter()
        .filter(|m| m.0.is_some())
        .map(|m| {
            let point1 = m.0.unwrap()[0];
            let point2 = m.0.unwrap()[1];
            format!(
                "({},{},{});({},{},{})\n",
                point1.x, point1.y, point1.z, point2.x, point2.y, point2.z
            )
        })
        .collect();

    let path = furnace.path.join(format!("Titanfall2/maps/{map}.furnace"));

    match fs::write(path, mesh_data) {
        Ok(_) => log::info!("created new furnace file"),
        Err(err) => log::error!("failed to creat new furnace file: {err}"),
    }
}

pub fn load_furnace_brush_data(furnace: &mut FurnaceData, map: String) {
    log::info!("loadingfurnace file");

    let path = furnace.path.join(format!("Titanfall2/maps/{map}.furnace"));

    let furnace_string_data = match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(err) => {
            log::error!("failed to load furnace data: {err}");
            None
        }
    };

    furnace.meshes = if let Some(furnace_string_data) = furnace_string_data {
        furnace_string_data
            .split('\n')
            .enumerate()
            .map(|line| {
                let points: Vec<Vector3> = line
                    .1
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

                (Some([points[0], points[1]]), line.0 as u32)
            })
            .collect()
    } else {
        Vec::new()
    }
}
