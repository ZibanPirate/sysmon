use anyhow::Result;
use tauri::command;

#[command]
pub fn get_current_screen_id_set() -> Result<Vec<String>, String> {
    #[cfg(target_os = "macos")]
    let screen_set = lib_swift::get_screen_info();
    #[cfg(target_os = "windows")]
    let screen_set = lib_cpp::get_screen_info();

    let screen_id_set: Vec<String> = screen_set.iter().map(|screen| screen.id.clone()).collect();

    Ok(screen_id_set)
}
