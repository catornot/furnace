#![allow(dead_code)]

use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::{fs, thread};

use mesh::{mesh_register_sqfunction, Mesh};

use rrplug::prelude::*;
use rrplug::wrappers::northstar::ScriptVmType;
use rrplug::{
    bindings::convar::FCVAR_GAMEDLL,
    concommand, sq_return_null, sqfunction,
    wrappers::northstar::{EngineLoadType, PluginData},
    OnceCell,
};

use crate::map_write::write_map_file;

mod map_write;
mod mesh;

pub struct FurnaceData {
    pub path: PathBuf,
    pub path_compiler: PathBuf,
    pub meshes: Vec<Mesh>,
    pub current_map: String,
}

pub static FURNACE: OnceCell<Mutex<FurnaceData>> = OnceCell::new();

#[derive(Debug)]
pub struct FurnacePlugin;

impl Plugin for FurnacePlugin {
    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self, plugin_data: &PluginData) {
        _ = plugin_data.register_sq_functions(info_push_map_name);

        mesh_register_sqfunction(plugin_data);

        _ = FURNACE.set(Mutex::new(FurnaceData {
            path: PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Titanfall2/R2Northstar/mods/cat_or_not.Furnace/mod/maps/compile/"),
            path_compiler: PathBuf::from("C:/Users/Alex/Desktop/apps/MRVN-radiant/remap.exe"),
            meshes: Vec::new(),
            current_map: "mp_default".into(),
        }));
    }

    fn main(&self) {}

    fn on_engine_load(&self, engine: EngineLoadType) {
        let engine = match engine {
            EngineLoadType::Engine(engine) => engine,
            EngineLoadType::EngineFailed => return,
            EngineLoadType::Server => return,
            EngineLoadType::Client => return,
        };

        // let convar = ConVarStruct::try_new().unwrap();
        // let register_info = ConVarRegister {
        //     callback: Some(basic_convar_changed_callback),
        //     ..ConVarRegister::mandatory(
        //         "basic_convar",
        //         "48",
        //         FCVAR_GAMEDLL.try_into().unwrap(),
        //         "basic_convar",
        //     )
        // };

        // convar.register(register_info).unwrap();

        _ = engine.register_concommand(
            "compile_map",
            compile_map_callback,
            "compiles the furnace map",
            FCVAR_GAMEDLL.try_into().unwrap(),
        );
    }

    fn on_sqvm_destroyed(&self, context: northstar::ScriptVmType) {
        if context != ScriptVmType::Server {
            return;
        }

        let mut furnace = FURNACE.wait().lock().unwrap();

        let map = format!("{}.map", &furnace.current_map);

        write_map_file(&mut furnace, map);
    }
}

#[concommand]
fn compile_map_callback(command: CCommandResult) {
    log::info!("running compile_map_callback");

    let mut furnace = match FURNACE.wait().lock() {
        Ok(f) => f,
        Err(e) => {
            log::error!("compilation failed {e}");
            return;
        }
    };

    let map = match command.args.get(0) {
        Some(arg) => arg,
        None => {
            log::warn!("map wasn't specified; defaulting to saved map name");
            &furnace.current_map
        }
    }
    .clone();

    write_map_file(&mut furnace, format!("{map}.map"));

    let compiler = &furnace.path_compiler;
    let basepath = furnace.path.clone();
    let path = furnace.path.join(format!("Titanfall2/maps/{map}.map"));

    log::info!("compiling {map}");

    // "C:/Users/Alex/Desktop/apps/MRVN-radiant/remap.exe"
    // -v -connect 127.0.0.1:39000
    // -game titanfall2
    // -fs_basepath "C:/Program Files (x86)/Steam/steamapps/common/Titanfall2/created_maps/"
    // -fs_game Titanfall2
    // -meta "C:/Program Files (x86)/Steam/steamapps/common/Titanfall2/created_maps/Titanfall2/maps/mp_default.map"

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
            _ = thread::spawn(|| match child.wait_with_output() {
                Ok(out) => {
                    log::info!("compilation finished {}", out.status);
                    copy_bsp(map)
                }
                Err(err) => log::error!("compilation falied: command execution fail, {err:?}"),
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

// #[convar]
// fn basic_convar_changed_callback(convar: Option<ConVarStruct>, old_value: String, float_old_value: f32) {
//     log::info!("old value: {}", float_old_value)
// }

#[sqfunction(VM=SERVER,ExportName=PushMapName)]
fn push_map_name(map_name: String) {
    let mut furnace = FURNACE.wait().lock().unwrap();
    furnace.current_map = map_name;

    sq_return_null!()
}

entry!(FurnacePlugin);
