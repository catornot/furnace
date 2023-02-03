use rrplug::OnceCell;
use std::sync::Mutex;

pub mod func_reg;
pub mod window;

pub static WINDOW_GLOBAL_DATA: OnceCell<Mutex<WindowGlobalData>> = OnceCell::new();

#[derive(Debug, Default)]
pub struct WindowGlobalData {
    grid: f32,
    eye_distance: f32
}
