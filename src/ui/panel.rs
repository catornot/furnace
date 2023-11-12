use hudhook::hooks::{dx11::ImguiDx11Hooks, ImguiRenderLoop};
use hudhook::*;
use imgui::*;
use rrplug::prelude::*;
use rrplug::{
    async_call_sq_function as async_call_sq_function_macro, high::squirrel::async_call_sq_function,
};

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

hudhook!(FurnacePanel.into_hook::<ImguiDx11Hooks>());

#[derive(Default)]
struct FurnacePanel;

#[inline(always)]
fn alias<T: FnOnce(*mut HSquirrelVM, &'static SquirrelFunctionsUnwraped) -> i32>(
    _: T,
) -> Option<T> {
    None
}

const fn alias_func(_: *mut HSquirrelVM, _: &'static SquirrelFunctionsUnwraped) -> i32 {
    unimplemented!()
}

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
                        async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NewGrid", grid);                        
                    }
                });

                ui.group(|| {
                    ui.input_float("Eye Dis", &mut window_data.eye_distance).build();

                    ui.same_line();

                    if ui.button("Push_") {                        
                        let eye_distance= window_data.eye_distance;
                        async_call_sq_function_macro!(
                            ScriptVmType::Ui,
                            "FurnaceCallBack_NewEyeDistance",
                            eye_distance
                        )
                    }
                });

                if ui.button("Snap To Closest Node") {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_InstantSnap", alias(alias_func)   )
                }

                if ui.button("Create New Brush") {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewBrush", alias(alias_func))
                }

                if ui.button("Create New Brush ( 2 points )") {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewBrushStaged", alias(alias_func))
                }

                ui.separator();

                ui.enabled(window_data.mesh_id.is_some(), || {
                    if ui.collapsing_header(
                        format!("mesh {}", window_data.mesh_id.unwrap_or_default()), TreeNodeFlags::empty(),
                    ){
                            let mesh_id = window_data.mesh_id.unwrap_or_default();
                            if ui.button("Delete") {                                                    
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_DeleteMesh", mesh_id)
                            }

                            ui.text(
                                "textures are not networked; only the server knowns the true texture used",
                            );

                            ui.group(|| {
                                ui.input_text("Texture",&mut window_data.texture).build();

                                ui.same_line();

                                if ui.button("Push__") {
                                let texture = window_data.texture.clone();
                                async_call_sq_function_macro!(
                                        ScriptVmType::Ui,
                                        "FurnaceCallBack_NewTexture",
                                        mesh_id,
                                        texture
                                    );
                                }
                            });

                            ui.input_float("Nudge Amount", &mut window_data.nudge).build();

                            let nudge = window_data.nudge;

                            if ui.button("Nudge +Z") {
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NudgeZUp", mesh_id, nudge)
                            }

                            if ui.button("Nudge -Z") {
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NudgeZDown", mesh_id, nudge)
                            }

                            if ui.button("Nudge +Y") {
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NudgeYUp", mesh_id, nudge)
                            }

                            if ui.button("Nudge -Y") {
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NudgeYDown", mesh_id, nudge)
                            }

                            if ui.button("Nudge +X") {
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NudgeXUp", mesh_id, nudge)
                            }

                            if ui.button("Nudge -X") {
                                async_call_sq_function_macro!(ScriptVmType::Ui, "FurnaceCallBack_NudgeXDown", mesh_id, nudge)
                            }
                        }
                });
            });
    }
}
