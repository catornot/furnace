use std::sync::Mutex;
use rrplug::OnceCell;

pub mod window;
pub mod func_reg;

pub static WINDOW_GLOBAL_DATA: OnceCell<Mutex<WindowGlobalData>> = OnceCell::new();

#[derive(Debug,Default)]
pub struct WindowGlobalData {
    grid: f32,
}
