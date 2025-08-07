use crate::settings::command::observe_settings;
use anyhow::Result;
use common_types::settings::Settings;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

pub fn setup_autostart<'a>(app: &'a mut tauri::App) -> Result<()> {
    if cfg!(debug_assertions) {
        println!("Debug mode: skipping autostart setup");
        return Ok(());
    }

    app.handle().plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        None,
    ))?;

    fn configure(app_handle: tauri::AppHandle) -> Result<()> {
        let settings_lock = app_handle.state::<Mutex<Settings>>();
        let settings = settings_lock.lock().unwrap();

        let autostart_manager = app_handle.autolaunch();

        if settings.general.start_on_boot {
            autostart_manager.enable()?;
        } else {
            autostart_manager.disable()?;
        }

        Ok(())
    }

    configure(app.handle().clone())?;

    observe_settings(configure);

    Ok(())
}
