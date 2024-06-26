use crate::map_info::{get_path_texture, write_furnace_brush_data, write_map_file};
use crate::{mesh::mesh_to_brush, FURNACE};
use once_cell::sync::Lazy;
use rrplug::high::engine_sync::{async_execute, AsyncEngineMessage};
use rrplug::prelude::*;
use std::{fs, process::Command, sync::Arc, thread};

static DEFAULT_TEXTURE: Lazy<Arc<str>> = Lazy::new(|| "world/dev/dev_white_512".to_string().into());

pub fn compile_map(context: ScriptContext) {
    let mut furnace = match FURNACE.wait().lock() {
        Ok(f) => f,
        Err(e) => {
            log::error!("compilation failed {e}");
            return;
        }
    };

    let map = furnace.current_map.clone();

    furnace.brushes.clear();

    furnace.brushes = furnace
        .meshes
        .iter()
        .filter_map(|m| m.as_ref())
        .enumerate()
        .map(|points| {
            mesh_to_brush(
                points.1[0],
                points.1[1],
                get_path_texture(
                    furnace
                        .texture_map
                        .get(points.0)
                        .unwrap_or(&DEFAULT_TEXTURE),
                ),
            )
        })
        .collect();

    write_map_file(&furnace, format!("{map}.map"));
    write_furnace_brush_data(&furnace, map.clone());

    let compiler = &furnace.path_compiler;
    let basepath = &furnace.path;
    let path = basepath.join(format!("Titanfall2/maps/{map}.map"));

    log::info!("compiling {map}");

    match Command::new(format!("{}", compiler.display()))
        .args([
            "-v".into(),
            "-connect".into(),
            "127.0.0.1:39000".into(),
            "-game".into(),
            "titanfall2".into(),
            "-fs_basepath".into(),
            basepath.display().to_string(),
            "-fs_game".into(),
            "Titanfall2".into(),
            "-meta".into(),
            format!("{}", path.display()),
        ])
        .spawn()
    {
        Ok(child) => {
            _ = thread::spawn(move || match child.wait_with_output() {
                Ok(out) => {
                    log::info!("compilation finished {}", out.status);
                    copy_bsp(map);

                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_ComfirmedCompilationEnded",
                        context,
                        (),
                    ));
                }
                Err(err) => log::error!("compilation failed: command execution fail, {err:?}"),
            })
        }
        Err(err) => log::error!("compilation falied: command not sent, {err:?}"),
    }
}

fn copy_bsp(map_name: String) {
    log::info!("copying bsp to cat_or_not.Furnace/mod/maps");

    let furnace = match FURNACE.wait().lock() {
        Ok(f) => f,
        Err(e) => {
            log::error!("compilation falied {e}");
            return;
        }
    };

    let mut path_maps = furnace.path.clone();
    path_maps.pop();

    let map = format!("{map_name}.bsp");

    log::info!("copying {map} to {}", path_maps.display());

    match fs::remove_file(path_maps.join(&map)) {
        Ok(_) => log::info!("removed old bsp"),
        Err(err) => log::error!("failed to remove old bsp: {err}"),
    }

    match fs::copy(
        furnace.path.join(format!("Titanfall2/maps/{}", &map)),
        path_maps.join(map),
    ) {
        Ok(_) => log::info!("copied bsp to maps folder"),
        Err(err) => log::error!("failed to copy bsp to maps folder: {err}"),
    }
}
