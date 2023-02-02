use rrplug::{prelude::*, sq_return_float, sqfunction};

use crate::ui::WINDOW_GLOBAL_DATA;

pub fn ui_register_sqfunction(plugin_data: &PluginData) {
    _ = plugin_data.register_sq_functions(info_get_grid);
}

#[sqfunction(VM=UI,ExportName=FurnaceGetGrid)]
fn get_grid() -> f32 {
    let window_data = WINDOW_GLOBAL_DATA.wait().lock().unwrap();

    sq_return_float!(window_data.grid, sqvm, sq_functions);
}
