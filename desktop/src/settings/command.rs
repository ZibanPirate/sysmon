use std::sync::Mutex;

use common_types::settings::Settings;
use tauri::{State, command};

#[command]
pub fn get_settings(settings_state: State<'_, Mutex<Settings>>) -> Result<Settings, String> {
    // todo-zm: persist settings to disk
    Ok(settings_state.lock().unwrap().to_owned())
}
