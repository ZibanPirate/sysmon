use anyhow::{Ok, Result};
use std::sync::Mutex;
use tauri::Manager;

use crate::settings::persist::load_settings;

pub fn setup_settings_state<'a>(app: &'a mut tauri::App) -> Result<()> {
    let loaded_settings = load_settings(app.handle())?;
    app.manage(Mutex::new(loaded_settings));

    Ok(())
}
