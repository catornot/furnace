use hudhook::hooks::{dx11::ImguiDx11Hooks, ImguiRenderLoop, ImguiRenderLoopFlags};
use hudhook::reexports::*;
use hudhook::*;
use imgui::*;
use rrplug::prelude::wait;
use rrplug::wrappers::{northstar::ScriptVmType, squirrel::async_call_sq_function};

use super::WINDOW_GLOBAL_DATA;

#[no_mangle]
#[export_name = "DllMain"]
pub unsafe extern "stdcall" fn dll_main(hmodule: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        hudhook::lifecycle::global_state::set_module(hmodule);

        init_hudhook();
    }
}

pub fn init_hudhook() {
    std::thread::spawn(move || {
        wait(100);
        let hooks: Box<dyn hooks::Hooks> = FurnacePanel::new().into_hook::<ImguiDx11Hooks>();

        unsafe { hooks.hook() };
        hudhook::lifecycle::global_state::set_hooks(hooks);
    });
}

#[derive(Default)]
struct FurnacePanel;

impl FurnacePanel {
    fn new() -> Self {
        Self {}
    }
}

impl ImguiRenderLoop for FurnacePanel {
    fn render(&mut self, ui: &mut Ui, _flags: &ImguiRenderLoopFlags) {
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
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewGrid", None)
                    }
                });

                ui.group(|| {
                    ui.input_float("Eye Dis", &mut window_data.eye_distance).build();

                    ui.same_line();

                    if ui.button("Push_") {
                        async_call_sq_function(
                            ScriptVmType::Ui,
                            "FurnaceCallBack_NewEyeDistance",
                            None,
                        )
                    }
                });

                if ui.button("Snap To Closest Node") {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_InstantSnap", None)
                }

                if ui.button("Create New Brush") {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewBrush", None)
                }

                if ui.button("Create New Brush ( 2 points )") {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewBrushStaged", None)
                }

                ui.separator();

                ui.enabled(window_data.mesh_id.is_some(), || {
                    if ui.collapsing_header(
                        format!("mesh {}", window_data.mesh_id.unwrap_or_default()), TreeNodeFlags::empty(),
                    ){
                            if ui.button("Delete") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_DeleteMesh", None)
                            }

                            ui.text(
                                "textures are not networked; only the server knowns the true texture used",
                            );

                            ui.group(|| {
                                
                                ui.input_text("Texture",&mut window_data.texture).build();

                                ui.same_line();

                                if ui.button("Push__") {
                                    async_call_sq_function(
                                        ScriptVmType::Ui,
                                        "FurnaceCallBack_NewTexture",
                                        None,
                                    );
                                }
                            });

                            ui.input_float("Nudge Amount", &mut window_data.nudge).build();

                            if ui.button("Nudge +Z") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeZUp", None)
                            }

                            if ui.button("Nudge -Z") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeZDown", None)
                            }

                            if ui.button("Nudge +Y") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeYUp", None)
                            }

                            if ui.button("Nudge -Y") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeYDown", None)
                            }

                            if ui.button("Nudge +X") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeXUp", None)
                            }

                            if ui.button("Nudge -X") {
                                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeXDown", None)
                            }
                        }
                    
                });
            });
    }
}
