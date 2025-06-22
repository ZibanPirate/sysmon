use common_types::settings::Settings;
use std::sync::Mutex;
use tauri::{State, command};

#[command]
pub fn get_settings(settings_state: State<'_, Mutex<Settings>>) -> Result<Settings, String> {
    // todo-zm: persist settings to disk
    Ok(settings_state.lock().unwrap().to_owned())
}

// todo-zm: callback all observers of settings changes then persist settings to disk
#[command]
pub fn set_settings(
    updated_settings: Settings,
    settings_state: State<'_, Mutex<Settings>>,
) -> Result<Settings, String> {
    let mut settings = settings_state.lock().unwrap();
    *settings = updated_settings;
    println!("Settings updated: {:?}", settings);
    Ok(settings.to_owned())
}
