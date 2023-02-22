use eframe::{egui, epaint::Vec2, EventLoopBuilderHook, RequestRepaintEvent};
use egui_winit::winit::{
    event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows,
};
use rrplug::{wrappers::{northstar::ScriptVmType, squirrel::async_call_sq_function}};

use super::WINDOW_GLOBAL_DATA;

pub fn init_window() {
    let func = |event_loop_builder: &mut EventLoopBuilder<RequestRepaintEvent>| {
        event_loop_builder.with_any_thread(true);
    };

    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(func));

    let options = eframe::NativeOptions {
        always_on_top: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: Some(Vec2::new(500.0, 400.0)),
        resizable: true,
        follow_system_theme: false,
        run_and_return: false,
        event_loop_builder,

        ..Default::default()
    };

    eframe::run_native(
        "Furnace Editor",
        options,
        Box::new(move |_cc| Box::new(Window::new())),
    );
}

struct Window {
    grid: String,
    eye_distance: String,
    nudge: String,
}

impl Window {
    fn new() -> Self {
        Self {
            grid: String::from("16"),
            eye_distance: String::from("1000"),
            nudge: String::from("1"),
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();
            
            ui.centered(|ui| {
                ui.heading("Furnace Editor");
                ui.end_row();
                ui.small("I think this some sort of ui panel for furnace");
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Grid");
                ui.text_edit_singleline(&mut self.grid);

                match self.grid.parse::<f32>() {
                    Ok(grid) => {
                        window_data.grid = grid;
                    }
                    Err(_) => _ = ui.small("this isn't a number :("),
                }

                if ui.button("Push").clicked() {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewGrid", None)
                }
            });

            ui.horizontal(|ui| {
                ui.label("Eye Dis");
                ui.text_edit_singleline(&mut self.eye_distance);

                match self.eye_distance.parse::<f32>() {
                    Ok(eye_distance) => {
                        window_data.eye_distance = eye_distance;
                    }
                    Err(_) => _ = ui.small("this isn't a number :("),
                }

                if ui.button("Push").clicked() {
                    async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewEyeDistance", None)
                }
            });

            if ui.button("Snap To Closest Node").clicked() {
                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_InstantSnap", None)
            }

            if ui.button("Create New Brush").clicked() {
                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewBrush", None)
            }
            
            if ui.button("Create New Brush ( 2 points )").clicked() {
                async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewBrushStaged", None)
            }

            ui.add_space(5.0);

            ui.add_visible_ui(window_data.mesh_id.is_some(), |ui| {
                ui.collapsing(format!("mesh {}", window_data.mesh_id.unwrap_or_default()), |ui| {
                    if ui.button("Delete").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_DeleteMesh", None)
                    }
                    
                    ui.small("textures are not networked; only the server knowns the true texture used");
                    ui.horizontal(|ui| {
                        ui.label("Texture");
                        ui.text_edit_singleline(&mut window_data.texture);
        
                        if ui.button("Push").clicked() {
                            async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewTexture", None );
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Nudge Amount");
                        ui.text_edit_singleline(&mut self.nudge);
        
                        match self.nudge.parse::<f32>() {
                            Ok(nudge) => {
                                window_data.nudge = nudge;
                            }
                            Err(_) => _ = ui.small("this isn't a number :("),
                        }
                    });

                    if ui.button("Nudge +Z").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeZUp", None)
                    }

                    if ui.button("Nudge -Z").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeZDown", None)
                    }

                    if ui.button("Nudge +Y").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeYUp", None)
                    }

                    if ui.button("Nudge -Y").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeYDown", None)
                    }

                    if ui.button("Nudge +X").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeXUp", None)
                    }

                    if ui.button("Nudge -X").clicked() {
                        async_call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NudgeXDown", None)
                    }
                })
            });
        });
    }
    fn on_close_event(&mut self) -> bool {
        false
    }
}