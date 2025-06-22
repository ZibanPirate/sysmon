use crate::settings::{command::observe_settings, persist::load_settings};
use anyhow::{Ok, Result};
use common_types::settings::{Settings, SettingsEvent};
use std::sync::Mutex;
use tauri::{Emitter, Manager};

pub fn setup_settings_state<'a>(app: &'a mut tauri::App) -> Result<()> {
    let loaded_settings = load_settings(app.handle())?;
    app.manage(Mutex::new(loaded_settings));

    observe_settings(|app_handle| {
        let settings_lock = app_handle.state::<Mutex<Settings>>();
        let settings = settings_lock.lock().unwrap();

        app_handle.emit("settings_changed", SettingsEvent::new(settings.to_owned()))?;

        Ok(())
    });

    Ok(())
}
