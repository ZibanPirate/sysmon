// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitor;
use monitor::register_monitor_for_window;
use tauri::{LogicalSize, WebviewWindowBuilder};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_positioner::{Position, WindowExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn resize(window: tauri::Window, width: f64, height: f64) {
    window.set_size(LogicalSize::new(width, height)).unwrap();
    window.move_window(Position::TopRight).unwrap();
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([Target::new(TargetKind::Stdout)])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![resize])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let autostart_manager = app.autolaunch();
            // Enable autostart
            let _ = autostart_manager.enable();
            // Check enable state
            println!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );

            let widget_window_builder = WebviewWindowBuilder::new(
                app,
                "widget",
                tauri::WebviewUrl::App("index.html".into()),
            );

            let widget_window_builder = widget_window_builder
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .inner_size(200.0, 50.0)
                .shadow(false)
                .title_bar_style(tauri::TitleBarStyle::Transparent);

            let widget_window = widget_window_builder.build()?;
            widget_window
                .as_ref()
                .window()
                .move_window(Position::TopRight)
                .unwrap();

            widget_window.set_ignore_cursor_events(true).unwrap();

            widget_window.on_window_event(|event| {
                println!("{:?}", event);
            });

            // widget_window.open_devtools();

            register_monitor_for_window(widget_window.as_ref().window());

            app.on_tray_icon_event(move |_, _| match widget_window.is_visible().unwrap() {
                true => widget_window.hide().unwrap(),
                false => {
                    widget_window
                        .as_ref()
                        .window()
                        .move_window(Position::TopRight)
                        .unwrap();
                    widget_window.show().unwrap();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
