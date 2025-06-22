use crate::settings::persist::persist_settings;
use anyhow::Result;
use common_types::settings::Settings;
use std::sync::{LazyLock, Mutex};
use tauri::{AppHandle, State, command};

#[command]
pub fn get_settings(settings_state: State<'_, Mutex<Settings>>) -> Result<Settings, String> {
    // todo-zm: persist settings to disk
    Ok(settings_state.lock().unwrap().to_owned())
}

// todo-zm: persist settings to disk
#[command]
pub fn set_settings(
    updated_settings: Settings,
    settings_state: State<'_, Mutex<Settings>>,
    app_handle: tauri::AppHandle,
) -> Result<Settings, String> {
    let mut settings = settings_state.lock().unwrap();
    *settings = updated_settings.clone();
    drop(settings);

    for observer in SETTINGS_OBSERVERS.lock().unwrap().iter() {
        if let Err(e) = observer(app_handle.clone()) {
            // todo-zm: report error
            eprintln!("Error notifying observer: {}", e);
        }
    }

    persist_settings(&updated_settings, app_handle)
        .map_err(|e| format!("Failed to persist settings: {}", e))?;

    Ok(updated_settings)
}

static SETTINGS_OBSERVERS: LazyLock<Mutex<Vec<Box<dyn Fn(AppHandle) -> Result<()> + Send>>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn observe_settings<T: Fn(AppHandle) -> Result<()> + Send + 'static>(callback: T) {
    SETTINGS_OBSERVERS.lock().unwrap().push(Box::new(callback));
}
