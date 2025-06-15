use crate::widget::monitor::start_monitoring;
use anyhow::Result;
use tauri::WebviewWindowBuilder;

// todo-zm: react to changes in settings
pub fn setup_widget<'a>(app: &'a mut tauri::App) -> Result<()> {
    WebviewWindowBuilder::new(app, "widget", tauri::WebviewUrl::App("index.html".into()))
        .always_on_top(true)
        .build()?;

    tokio::spawn(start_monitoring(app.handle().clone()));

    Ok(())
}
