mod autostart;
mod monitor;
mod settings;
mod tray;
mod updater;
mod utils;
mod widget;

use autostart::setup_autostart;
use monitor::setup_monitoring;
use settings::{load_settings, SettingsState};
use std::sync::Mutex;
use tauri::{Emitter, LogicalSize, State};
use tauri_plugin_autostart::MacosLauncher;
use tray::setup_tray;
use updater::setup_updater;
use utils::StateSubscriber;
use widget::{setup_widget, WidgetWindow};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn resize(window: tauri::Window, width: f64, height: f64, store: State<Store>) {
    let settings_state = store
        .inner()
        .settings
        .lock()
        .map_err(|_| "Failed to lock settings")
        .expect("Failed to lock settings");
    let state = settings_state.get_state();

    window
        .emit("settings", state.clone())
        .expect("Failed to emit settings");

    window
        .set_size(LogicalSize::new(width, height))
        .expect("Failed to set window size");

    window
        .move_widget(&state.widget_position, state.safe_area)
        .expect("Failed to move window");
}

#[derive(Default, Debug)]
struct Store {
    settings: Mutex<SettingsState>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::AppleScript,
            None,
        ))
        .invoke_handler(tauri::generate_handler![resize])
        .manage(Store::default())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            load_settings(app)?;
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
