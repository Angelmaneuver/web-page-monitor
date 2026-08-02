use serde::{Deserialize, Serialize};
use std::panic;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use tauri::utils::config::WindowConfig;
use tauri::{AppHandle, Position, Url, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_cli::CliExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

mod window;

static BACKEND_READY: AtomicBool = AtomicBool::new(false);
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
                kiosk: false,
                position: None,
            };

            set_cli_args(app, &mut config);

            window::CONFIG.set(config.clone()).ok();

            let app_handle = app.handle();

            let main = WebviewWindowBuilder::from_config(app_handle, &config.main)?.build()?;
            let monitor = WebviewWindowBuilder::from_config(app_handle, &config.monitor)?
                .parent(&main)?
                .build()?;

            if let Some((x, y)) = config.position {
                let default_position = main.inner_position().ok().and_then(|position| {
                    main.scale_factor()
                        .ok()
                        .map(|scale_factor| position.to_logical::<f64>(scale_factor))
                });

                let x =
                    x.unwrap_or_else(|| default_position.map(|position| position.x).unwrap_or(0.0));
                let y =
                    y.unwrap_or_else(|| default_position.map(|position| position.y).unwrap_or(0.0));

                main.set_position(Position::Logical(tauri::LogicalPosition::new(x, y)))
                    .ok();
            }

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

            BACKEND_READY.store(true, Ordering::SeqCst);

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            is_backend_ready,
            get_initial_data,
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

        let kiosk = matches
            .args
            .get("kiosk")
            .and_then(|arg| arg.value.as_bool())
            .unwrap_or(false);
        dbg!("{:?}", &kiosk);
        config.kiosk = kiosk;

        let x = matches.args.get("x").and_then(|arg| arg.value.as_str());
        let y = matches.args.get("y").and_then(|arg| arg.value.as_str());

        config.position = parse_position_args(x, y);
    } else {
        panic!("error while analyzing the startup options");
    }
}

fn parse_position_args(x: Option<&str>, y: Option<&str>) -> Option<(Option<f64>, Option<f64>)> {
    match (x, y) {
        (Some(x_value), Some(y_value)) => Some((
            Some(parse_coordinate_arg(x_value, "x")),
            Some(parse_coordinate_arg(y_value, "y")),
        )),
        (Some(x_value), None) => Some((Some(parse_coordinate_arg(x_value, "x")), None)),
        (None, Some(y_value)) => Some((None, Some(parse_coordinate_arg(y_value, "y")))),
        (None, None) => None,
    }
}

fn parse_coordinate_arg(value: &str, name: &str) -> f64 {
    value.parse::<f64>().unwrap_or_else(|_| {
        panic!("error while the {name} coordinate has not been specified or is invalid")
    })
}

#[tauri::command]
fn is_backend_ready() -> bool {
    return BACKEND_READY.load(Ordering::SeqCst);
}

#[derive(Debug, Serialize, Deserialize)]
struct Initial {
    pub label: String,
    pub kiosk: bool,
}

#[tauri::command]
fn get_initial_data() -> Initial {
    return Initial {
        label: window::WINDOW.get().unwrap().main.label().to_string(),
        kiosk: window::CONFIG.get().unwrap().kiosk,
    };
}

#[cfg(test)]
mod tests {
    use super::parse_position_args;

    #[test]
    fn parses_provided_coordinates() {
        assert_eq!(
            parse_position_args(Some("100.5"), Some("200.25")),
            Some((Some(100.5), Some(200.25)))
        );
    }

    #[test]
    fn uses_default_coordinate_when_x_is_not_provided() {
        assert_eq!(
            parse_position_args(None, Some("200.25")),
            Some((None, Some(200.25)))
        );
    }

    #[test]
    fn uses_default_coordinate_when_y_is_not_provided() {
        assert_eq!(
            parse_position_args(Some("100.5"), None),
            Some((Some(100.5), None))
        );
    }

    #[test]
    fn returns_none_when_coordinates_are_not_provided() {
        assert_eq!(parse_position_args(None, None), None);
    }

    #[test]
    #[should_panic(expected = "x")]
    fn panics_for_invalid_coordinate() {
        let _ = parse_position_args(Some("not-a-number"), None);
    }
}
