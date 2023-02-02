use eframe::{egui, epaint::Vec2, EventLoopBuilderHook, RequestRepaintEvent};
use egui_winit::winit::{
    event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows,
};
use rrplug::{
    wrappers::{northstar::ScriptVmType, squirrel::call_sq_function},
};

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
}

impl Window {
    fn new() -> Self {
        Self {
            grid: String::from("16"),
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
                    call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_NewGrid", None)
                }
            });



            if ui.button("Snap To Closest Node").clicked() {
                call_sq_function(ScriptVmType::Ui, "FurnaceCallBack_InstantSnap", None)
            }
            
        });
    }
    fn on_close_event(&mut self) -> bool {
        false
    }
}
