#![feature(
    int_roundings,
    // once_cell,
    // lazy_cell,
    // once_cell_try
)]

use dotenv::from_path;
use once_cell::sync::OnceCell;
use rrplug::bindings::cvar::convar::{FCVAR_CLIENTDLL, FCVAR_GAMEDLL};
use rrplug::prelude::*;
use std::{
    fs::{create_dir, remove_dir},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    client::func_reg::client_register_sqfunction,
    compile::compile_map,
    map_info::{write_furnace_brush_data, write_map_file, TEXTURE_MAP},
    mesh::Mesh,
    server::func_reg::sever_register_sqfunction,
    ui::{func_reg::ui_register_sqfunction, WindowGlobalData, WINDOW_GLOBAL_DATA},
};

mod client;
mod compile;
mod map_info;
mod mesh;
mod server;
mod ui;

#[derive(Debug)]
pub struct FurnaceData {
    pub path: PathBuf,
    pub path_compiler: PathBuf,
    pub brushes: Vec<Mesh>,
    pub meshes: Vec<Option<[Vector3; 2]>>,
    pub texture_map: Vec<Arc<str>>,
    pub current_map: String,
    pub last_compiled: String,
}

pub static FURNACE: OnceCell<Mutex<FurnaceData>> = OnceCell::new();

#[derive(Debug)]
pub struct FurnacePlugin;

impl Plugin for FurnacePlugin {
    fn new(plugin_data: &PluginData) -> Self {
        sever_register_sqfunction(plugin_data);
        client_register_sqfunction(plugin_data);
        ui_register_sqfunction(plugin_data);

        let paths = match from_path("R2Northstar/plugins/furnace.env") {
            Ok(_) => (
                PathBuf::from(std::env::var("PATH_MOD").expect("how")),
                PathBuf::from(std::env::var("PATH_COMPILER").expect("how")),
            ),
            Err(err) => {
                log::error!("{err}");
                wait(1000);
                panic!()
            }
        };

        log::info!("path mod : {}", paths.0.display());
        log::info!("path compiler : {}", paths.1.display());

        // for testing
        #[cfg(debug_assertions)]
        create_dir(paths.0.join("test")).unwrap_or_else(|err| {
            log::error!("{err}");
            wait(1000);
            panic!()
        });

        #[cfg(debug_assertions)]
        remove_dir(paths.0.join("test")).unwrap_or_else(|err| {
            log::error!("{err}");
            wait(1000);
            panic!()
        });

        _ = FURNACE.set(Mutex::new(FurnaceData {
            path: paths.0,
            path_compiler: paths.1,
            brushes: Vec::new(),
            meshes: Vec::new(),
            texture_map: Vec::new(),
            current_map: "mp_default".into(),
            last_compiled: "mp_default".into(),
        }));

        _ = WINDOW_GLOBAL_DATA.set(Mutex::new(WindowGlobalData::default()));

        Self {}
    }

    fn on_dll_load(&self, engine_data: Option<&EngineData>, _dll_ptr: &DLLPointer) {
        let Some(engine) = engine_data else {
            return;
        };
        _ = engine.register_concommand(
            "compile_map",
            compile_map_callback,
            "compiles the furnace map",
            FCVAR_GAMEDLL.try_into().unwrap(),
        );

        _ = engine.register_concommand(
            "dump_textures",
            dump_textures_callback,
            "gives all the shorthands of textures",
            FCVAR_CLIENTDLL.try_into().unwrap(),
        );
    }

    fn on_sqvm_destroyed(&self, context: ScriptVmType) {
        if context != ScriptVmType::Server {
            return;
        }

        let mut furnace = FURNACE.wait().lock().unwrap();

        let map_file = format!("{}.map", &furnace.current_map);
        write_map_file(&furnace, map_file);

        let map = furnace.current_map.to_owned();
        write_furnace_brush_data(&furnace, map);

        furnace.brushes.clear();
        furnace.meshes.clear();
    }
}

#[rrplug::concommand]
fn compile_map_callback() {
    compile_map(ScriptVmType::Server)
}

#[rrplug::concommand]
fn dump_textures_callback() {
    log::info!("List of textures!");

    for (key, value) in TEXTURE_MAP.iter() {
        log::info!("{key} : {value}")
    }
}

entry!(FurnacePlugin);
