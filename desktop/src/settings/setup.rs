use anyhow::{Ok, Result};
use common_types::settings::Settings;
use std::sync::Mutex;
use tauri::Manager;

pub fn setup_settings_state<'a>(app: &'a mut tauri::App) -> Result<()> {
    app.manage(Mutex::new(Settings::default()));

    Ok(())
}
