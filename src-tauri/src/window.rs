use std::sync::OnceLock;
use tauri::utils::config::WindowConfig;
use tauri::Size::Logical;
use tauri::{AppHandle, WebviewWindow};

pub struct Config {
    pub main: WindowConfig,
    pub monitor: WindowConfig,
}

pub struct Window {
    pub main: WebviewWindow,
    pub monitor: WebviewWindow,
}

pub static WINDOW: OnceLock<Window> = OnceLock::new();

pub fn resize_monitor() {
    let window = WINDOW.get().unwrap();
    let scale_factor = window.main.scale_factor().unwrap();

    let mut size = window
        .main
        .inner_size()
        .unwrap()
        .to_logical::<f64>(scale_factor);

    size.width = size.width - 0.5 - 0.5 - 0.5 - 0.5 - 10.0;
    size.height = size.height - 39.59 - 5.0 - 0.5 - 0.5 - 0.5 - 0.5 - 10.0;

    window.monitor.set_size(Logical(size)).unwrap();

    reset_position_monitor();
}

pub fn reset_position_monitor() {
    let window = WINDOW.get().unwrap();
    let scale_factor = window.main.scale_factor().unwrap();

    let mut position = window
        .main
        .inner_position()
        .unwrap()
        .to_logical::<f64>(scale_factor);

    position.x = position.x + 0.5 + 0.5 + 5.0;
    position.y = position.y + 39.59 + 5.0 + 0.5 + 0.5 + 4.0;

    window.monitor.set_position(position).unwrap();
}

#[tauri::command]
pub fn get_app_label() -> String {
    return WINDOW.get().unwrap().main.label().to_string();
}

#[tauri::command]
pub fn reload() {
    WINDOW.get().unwrap().monitor.reload().unwrap();
}

#[tauri::command]
pub fn exit(app: AppHandle) {
    #[cfg(target_os = "macos")]
    {
        app.hide().unwrap();
    }
    #[cfg(not(target_os = "macos"))]
    {
        app.exit(0);
    }
}
