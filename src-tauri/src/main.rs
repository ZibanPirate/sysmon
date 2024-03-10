// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WebviewWindowBuilder;
use tauri_plugin_positioner::{Position, WindowExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn close(window: tauri::Window) {
    window.close().unwrap();
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![close])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

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
                .title_bar_style(tauri::TitleBarStyle::Transparent);

            let widget_window = widget_window_builder.build()?;
            widget_window
                .as_ref()
                .window()
                .move_window(Position::TopRight)
                .unwrap();

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
