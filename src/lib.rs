#![feature(int_roundings)]

use std::fs::{create_dir, remove_dir};
use std::path::PathBuf;

use std::sync::Mutex;

use client::func_reg::client_register_sqfunction;
use compile::compile_map;
use dotenv::from_path;
use map_info::{write_furnace_brush_data, TEXTURE_MAP};
use mesh::Mesh;

use rrplug::bindings::convar::FCVAR_CLIENTDLL;
use rrplug::prelude::*;
use rrplug::wrappers::northstar::ScriptVmType;

use rrplug::wrappers::vector::Vector3;
use rrplug::{
    bindings::convar::FCVAR_GAMEDLL,
    concommand,
    wrappers::northstar::{EngineLoadType, PluginData},
    OnceCell,
};
use server::func_reg::sever_register_sqfunction;
use ui::window::init_window;

use crate::map_info::write_map_file;
use crate::ui::func_reg::ui_register_sqfunction;
use crate::ui::{WindowGlobalData, WINDOW_GLOBAL_DATA};

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
    pub texture_map: Vec<String>,
    pub current_map: String,
    pub last_compiled: String,
}

pub static FURNACE: OnceCell<Mutex<FurnaceData>> = OnceCell::new();

#[derive(Debug)]
pub struct FurnacePlugin;

impl Plugin for FurnacePlugin {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
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
    }

    fn main(&self) {
        init_window()
    }

    fn on_engine_load(&self, engine: EngineLoadType) {
        let engine = match engine {
            EngineLoadType::Engine(engine) => engine,
            EngineLoadType::EngineFailed => return,
            EngineLoadType::Server => return,
            EngineLoadType::Client => return,
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

    fn on_sqvm_destroyed(&self, context: northstar::ScriptVmType) {
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

#[concommand]
fn compile_map_callback(_command: CCommandResult) {
    compile_map(ScriptVmType::Server)
}

#[concommand]
fn dump_textures_callback(_command: CCommandResult) {
    log::info!("List of textures!");

    for (key, value) in TEXTURE_MAP.iter() {
        log::info!("{key} : {value}")
    }
}

entry!(FurnacePlugin);
