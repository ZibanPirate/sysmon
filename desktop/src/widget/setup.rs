use anyhow::Result;
use tauri::WebviewWindowBuilder;

pub fn setup_widget(app: &mut tauri::App) -> Result<()> {
    let network_widget_window_builder =
        WebviewWindowBuilder::new(app, "widget", tauri::WebviewUrl::App("index.html".into()));

    let _network_widget_window = network_widget_window_builder.build()?;

    // todo-zm: react to changes in settings
    // todo-zm: listen to telemetry events, should we rely on tauri commands for this?
    Ok(())
}
