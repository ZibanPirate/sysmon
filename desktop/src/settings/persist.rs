use anyhow::Result;
use common_types::settings::Settings;
use std::fs::create_dir_all;
use tauri::{AppHandle, Manager};

// todo-zm: make sure settings_path is the right place to store settings
pub fn persist_settings(settings: &Settings, app_handle: AppHandle) -> Result<()> {
    let config_dir = app_handle.path().config_dir()?;
    create_dir_all(&config_dir)?;
    let settings_path = config_dir.join("settings.json");
    let file = std::fs::File::create(&settings_path)?;
    serde_json::to_writer_pretty(file, settings)?;

    Ok(())
}

pub fn load_settings(app_handle: &AppHandle) -> Result<Settings> {
    let config_dir = app_handle.path().config_dir()?;
    let settings_path = config_dir.join("settings.json");
    if !settings_path.exists() {
        return Ok(Settings::default());
    }
    let file = std::fs::File::open(settings_path)?;
    let settings: Settings = serde_json::from_reader(file).unwrap_or_default();
    Ok(settings)
}
