use tauri::App;
use tauri_plugin_autostart::ManagerExt;

pub fn setup_autostart(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        println!("Skipping autostart setup because debug build");
        return Ok(());
    }

    let autostart_manager = app.autolaunch();
    autostart_manager
        .enable()
        .map_err(|e| format!("Failed to enable autostart: {:?}", e))?;

    Ok(())
}
