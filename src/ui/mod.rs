use rrplug::OnceCell;
use std::sync::Mutex;

pub mod func_reg;
pub mod window;

pub static WINDOW_GLOBAL_DATA: OnceCell<Mutex<WindowGlobalData>> = OnceCell::new();

#[derive(Debug)]
pub struct WindowGlobalData {
    pub grid: f32,
    pub eye_distance: f32,
    pub nudge: f32,
    pub mesh_id: Option<i32>,
    pub texture: String
}
impl Default for WindowGlobalData {
    fn default() -> Self {
        Self {
            grid: 16.0,
            eye_distance: 1000.0,
            nudge: 1.0,
            mesh_id: Default::default(),
            texture: String::from("$w"),
        }
    }
}
