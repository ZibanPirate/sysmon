// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitor;
mod settings;
mod tray;
mod update;
mod utils;

use monitor::register_monitor_for_window;
use settings::SettingsState;
use std::sync::{Arc, Mutex};
use tauri::{LogicalSize, Manager, WebviewWindowBuilder};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri_plugin_updater::UpdaterExt;
use tray::setup_tray;
use update::register_updater;
use utils::StateSubscriber;

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

            let store = app.state::<Store>();

            let mut lock = store.inner().settings.lock().unwrap();
            let state = lock.get_state();

            if !cfg!(debug_assertions) {
                let updater = app.updater().unwrap();
                register_updater(updater);

                let autostart_manager = app.autolaunch();
                let _ = autostart_manager.enable();
                println!(
                    "registered for autostart? {}",
                    autostart_manager.is_enabled().unwrap()
                );
            }

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
                .visible(state.show_widget)
                .title_bar_style(tauri::TitleBarStyle::Transparent);

            let widget_window = widget_window_builder.build()?;
            widget_window
                .as_ref()
                .window()
                .move_window(Position::TopRight)
                .unwrap();

            widget_window.set_ignore_cursor_events(true).unwrap();
            let widget_window = Arc::new(widget_window.as_ref().window());

            lock.set_state(settings::Settings {
                widget_window: Some(widget_window.clone()),
                ..state
            });

            lock.on_path_change(
                settings::SettingsPath::ShowWidget,
                Box::new(|new_state: &settings::Settings| {
                    println!("show widget changed to {:?}", new_state.show_widget);
                    match new_state.show_widget {
                        true => new_state.widget_window.as_ref().unwrap().show().unwrap(),
                        false => new_state.widget_window.as_ref().unwrap().hide().unwrap(),
                    }
                }),
            );

            register_monitor_for_window(widget_window);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
