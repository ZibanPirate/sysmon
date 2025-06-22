use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewWindowBuilder};

pub fn show_settings_window<'a>(app: &'a AppHandle) -> Result<()> {
    if let Some(settings_windows) = app.get_webview_window("settings") {
        settings_windows.set_focus()?;
        return Ok(());
    }

    WebviewWindowBuilder::new(
        app,
        "settings",
        tauri::WebviewUrl::App("src/_entries/settings.html".into()),
    )
    .title("Settings - Sysmon")
    .inner_size(400.0, 400.0)
    .build()?;

    Ok(())
}
