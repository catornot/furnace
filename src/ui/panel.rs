use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::*;
use imgui::*;
use rrplug::high::engine_sync::{async_execute, AsyncEngineMessage};
use rrplug::prelude::*;

use super::WINDOW_GLOBAL_DATA;

// static TYPE_DEFAULT_FUNC<T: Fn(*mut HSquirrelVM, SquirrelFunctionsUnwraped) >:
// = |_,_| {0};

// #[no_mangle]
// #[export_name = "DllMain"]
// pub unsafe extern "stdcall" fn dll_main(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
//     if reason == DLL_PROCESS_ATTACH {
//         hudhook::lifecycle::global_state::set_module(hmodule);

//         init_hudhook();
//     }
// }

// pub fn init_hudhook() {
//     std::thread::spawn(move || {
//         wait(100);
//         let hooks: Box<dyn hooks::Hooks> = FurnacePanel::new().into_hook::<ImguiDx11Hooks>();

//         unsafe { hooks.hook() };
//         hudhook::lifecycle::global_state::set_hooks(hooks);
//     });
// }

pub fn init_gui() {
    static mut INIT: bool = false;

    if unsafe { INIT } {
        return;
    }
    unsafe { INIT = true };

    if let Err(e) = Hudhook::builder()
        .with(unsafe { Box::new(ImguiDx11Hooks::new(FurnacePanel)) })
        // .with_hmodule(unsafe { windows::Win32::System::Threading::GetCurrentProcess() })
        .build()
        .apply()
    {
        log::error!("Couldn't apply hooks: {e:?}");
    }
}

#[derive(Default)]
struct FurnacePanel;

impl ImguiRenderLoop for FurnacePanel {
    fn render(&mut self, ui: &mut Ui) {
        ui.window("Furnace Editor")
            .collapsible(true)
            .resizable(true)
            .size([600., 450.], Condition::FirstUseEver)
            .build(|| {
                ui.set_window_font_scale(1.6);

                let mut window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

                ui.group(|| {
                    ui.input_float("Grid", &mut window_data.grid).build();

                    ui.same_line();

                    if ui.button("Push") {
                        let grid = window_data.grid;
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NewGrid",
                        ScriptContext::UI,
                        grid,
                    ));
                    }
                });

                ui.group(|| {
                    ui.input_float("Eye Dis", &mut window_data.eye_distance).build();

                    ui.same_line();

                    if ui.button("Push_") {                        
                        let eye_distance= window_data.eye_distance;
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NewEyeDistance",
                        ScriptContext::UI,
                        eye_distance
,
                    ));
                    }
                });

                if ui.button("Snap To Closest Node") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_InstantSnap",
                        ScriptContext::UI,
                        (),
                    ));
                }

                if ui.button("Create New Brush") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NewBrush",
                        ScriptContext::UI,
                        (),
                    ));
                }

                if ui.button("Create New Brush ( 2 points )") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NewBrushStaged",
                        ScriptContext::UI,
                        (),
                    ));
                }

                ui.separator();

                ui.enabled(window_data.mesh_id.is_some(), || {
                    if ui.collapsing_header(
                        format!("mesh {}", window_data.mesh_id.unwrap_or_default()), TreeNodeFlags::empty(),
                    ){
                            let mesh_id = window_data.mesh_id.unwrap_or_default();
                            if ui.button("Delete") {                                                    
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_DeleteMesh",
                        ScriptContext::UI,
                        mesh_id,
                    ));
                            }

                            ui.text(
                                "textures are not networked; only the server knowns the true texture used",
                            );

                            ui.group(|| {
                                ui.input_text("Texture",&mut window_data.texture).build();

                                ui.same_line();

                                if ui.button("Push__") {

                                let texture = window_data.texture.clone();
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NewTexture",
                        ScriptContext::UI,
                        (mesh_id, texture)
                    ));
                                }
                            });

                            ui.input_float("Nudge Amount", &mut window_data.nudge).build();

                            let nudge = window_data.nudge;

                            if ui.button("Nudge +Z") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NudgeZUp",
                        ScriptContext::UI,
                        (mesh_id, nudge)
                    ));
                            }

                            if ui.button("Nudge -Z") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NudgeZDown",
                        ScriptContext::UI,
                        (mesh_id, nudge)
                    ));
                            }

                            if ui.button("Nudge +Y") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NudgeYUp",
                        ScriptContext::UI,
                        (mesh_id, nudge)
                    ));
                            }

                            if ui.button("Nudge -Y") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NudgeYDown",
                        ScriptContext::UI,
                        (mesh_id, nudge)
                    ));
                            }

                            if ui.button("Nudge +X") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NudgeXUp",
                        ScriptContext::UI,
                        (mesh_id, nudge)
                    ));
                            }

                            if ui.button("Nudge -X") {
                    _ = async_execute(AsyncEngineMessage::run_squirrel_func(
                        "FurnaceCallBack_NudgeXDown",
                        ScriptContext::UI,
                        (mesh_id, nudge)
                    ));
                            }
                        }
                });
            });
    }
}
