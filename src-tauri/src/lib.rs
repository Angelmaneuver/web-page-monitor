use std::str::FromStr;

use tauri::utils::config::WindowConfig;
use tauri::{Url, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_cli::CliExt;

mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            let mut config = window::Config {
                main: get_config(app, "main"),
                monitor: get_config(app, "monitor"),
            };

            if let Ok(matches) = app.cli().matches() {
                dbg!("{:?}", &matches);

                if let Some(arg) = matches.args.get("label") {
                    if let Some(value) = arg.value.as_str() {
                        let label = value.to_string();
                        dbg!("{:?}", &label);

                        config.main.label = label.clone();
                        config.monitor.label = format!("{}_monitor", label);
                    }
                }

                if let Some(arg) = matches.args.get("url") {
                    if let Some(value) = arg.value.as_str() {
                        let url = Url::from_str(&value.to_string())?;
                        dbg!("{:?}", &url);

                        config.monitor.url = WebviewUrl::External(url);
                    } else {
                        panic!(
                            "error while URL for the monitioring web page has not been specified"
                        );
                    }
                }
            }

            let app_handle = app.handle();

            let main = WebviewWindowBuilder::from_config(app_handle, &config.main)?.build()?;
            let monitor = WebviewWindowBuilder::from_config(app_handle, &config.monitor)?
                .parent(&main)?
                .build()?;

            window::WINDOW.get_or_init(|| window::Window {
                main: main.clone(),
                monitor: monitor.clone(),
            });

            window::resize_monitor();

            main.on_window_event(move |event| {
                if let WindowEvent::Resized(_size) = event {
                    window::resize_monitor();
                } else if let WindowEvent::Moved(_position) = event {
                    window::reset_position_monitor();
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            window::get_app_label,
            window::reload,
            window::exit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_config(app: &mut tauri::App, label: &str) -> WindowConfig {
    return app
        .config()
        .app
        .windows
        .iter()
        .find(|&config| config.label == label)
        .expect(&format!("error while reading the {} window config", label))
        .clone();
}
