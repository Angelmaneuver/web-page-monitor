use std::panic;
use std::str::FromStr;
use std::sync::OnceLock;

use tauri::utils::config::WindowConfig;
use tauri::{AppHandle, Emitter, Url, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_cli::CliExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

mod window;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    set_hook();

    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).ok();

            let mut config = window::Config {
                main: get_window_config(app, "main"),
                monitor: get_window_config(app, "monitor"),
            };

            set_cli_args(app, &mut config);

            let app_handle = app.handle();

            let main = WebviewWindowBuilder::from_config(app_handle, &config.main)?.build()?;
            let monitor = WebviewWindowBuilder::from_config(app_handle, &config.monitor)?
                .parent(&main)?
                .build()?;

            window::WINDOW
                .set(window::Window {
                    main: main.clone(),
                    monitor: monitor.clone(),
                })
                .ok();

            window::resize_monitor();

            main.on_window_event(move |event| {
                if let WindowEvent::Resized(_size) = event {
                    window::resize_monitor();
                } else if let WindowEvent::Moved(_position) = event {
                    window::reset_position_monitor();
                }
            });

            app.emit("backend-ready", &config.main.label).ok();

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

fn set_hook() {
    panic::set_hook(Box::new(|info| {
        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or("unknown panic");

        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message_text = format!("panic occurred:\n{}\n{}", payload, location);

        if let Some(app_handle) = APP_HANDLE.get() {
            let _ = app_handle
                .dialog()
                .message(&message_text)
                .kind(MessageDialogKind::Error)
                .title("Error")
                .blocking_show();
        } else {
            eprintln!("{}", message_text);
        }
    }));
}

fn get_window_config(app: &mut tauri::App, label: &str) -> WindowConfig {
    return app
        .config()
        .app
        .windows
        .iter()
        .find(|&config| config.label == label)
        .expect(&format!("error while reading the {} window config", label))
        .clone();
}

fn set_cli_args(app: &mut tauri::App, config: &mut window::Config) {
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
                let url = Url::from_str(&value.to_string())
                    .expect("error while URL for the monitoring web page is invalid");
                dbg!("{:?}", &url);

                config.monitor.url = WebviewUrl::External(url);
            } else {
                panic!("error while URL for the monitioring web page has not been specified");
            }
        }
    } else {
        panic!("error while analyzing the startup options");
    }
}
