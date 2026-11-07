use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::utils::config::WindowConfig;
use tauri::Size::Logical;
use tauri::{AppHandle, WebviewWindow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub main: WindowConfig,
    pub monitor: WindowConfig,
    pub kiosk: bool,
    pub icon: Option<PathBuf>,
    pub position: Option<(Option<f64>, Option<f64>)>,
}

pub struct Window {
    pub main: WebviewWindow,
    pub monitor: WebviewWindow,
}

pub static WINDOW: OnceLock<Window> = OnceLock::new();
pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn resize_monitor() {
    let window = WINDOW.get().unwrap();
    let scale_factor = window.main.scale_factor().unwrap();

    let config = CONFIG.get().unwrap();

    let mut size = window
        .main
        .inner_size()
        .unwrap()
        .to_logical::<f64>(scale_factor);

    size.width = size.width - 0.5 - 0.5 - 0.5 - 0.5 - 10.0;

    if config.kiosk {
        size.height = size.height - 0.5 - 0.5 - 0.5 - 0.5 - 10.0;
    } else {
        size.height = size.height - 39.59 - 5.0 - 0.5 - 0.5 - 0.5 - 0.5 - 10.0;
    }

    window.monitor.set_size(Logical(size)).unwrap();

    reset_position_monitor();
}

pub fn reset_position_monitor() {
    let window = WINDOW.get().unwrap();
    let scale_factor = window.main.scale_factor().unwrap();

    let config = CONFIG.get().unwrap();

    let mut position = window
        .main
        .inner_position()
        .unwrap()
        .to_logical::<f64>(scale_factor);

    position.x = position.x + 0.5 + 0.5 + 5.0;

    if config.kiosk {
        position.y = position.y + 0.5 + 0.5 + 4.0;
    } else {
        position.y = position.y + 39.59 + 5.0 + 0.5 + 0.5 + 4.0;
    }

    window.monitor.set_position(position).unwrap();
}

#[tauri::command]
pub fn reload() {
    WINDOW.get().unwrap().monitor.reload().unwrap();
}

pub fn show_on_top() -> tauri::Result<()> {
    let main = &WINDOW.get().unwrap().main;

    main.show()?;
    main.unminimize()?;
    main.set_focus()?;

    Ok(())
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
