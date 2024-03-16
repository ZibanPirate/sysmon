// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart;
mod monitor;
mod settings;
mod tray;
mod updater;
mod utils;
mod widget;

use autostart::setup_autostart;
use monitor::setup_monitoring;
use settings::SettingsState;
use std::sync::Mutex;
use tauri::LogicalSize;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_positioner::{Position, WindowExt};
use tray::setup_tray;
use updater::setup_updater;
use widget::setup_widget;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn resize(window: tauri::Window, width: f64, height: f64) {
    window.set_size(LogicalSize::new(width, height)).unwrap();
    window.move_window(Position::TopRight).unwrap();
}

#[derive(Default, Debug)]
struct Store {
    settings: Mutex<SettingsState>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::AppleScript,
            None,
        ))
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![resize])
        .manage(Store::default())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            setup_tray(app)?;
            setup_widget(app)?;
            setup_monitoring(app)?;
            setup_updater(app)?;
            setup_autostart(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
