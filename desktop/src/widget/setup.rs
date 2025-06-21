use crate::widget::monitor::start_monitoring;
use anyhow::Result;
use lib_swift::get_screen_info;
use tauri::{LogicalPosition, Position, WebviewWindowBuilder};

// todo-zm: react to changes in settings
pub fn setup_widget<'a>(app: &'a mut tauri::App) -> Result<()> {
    let widget_window =
        WebviewWindowBuilder::new(app, "widget", tauri::WebviewUrl::App("index.html".into()))
            .always_on_top(true)
            .inner_size(200.0, 50.0)
            .resizable(false)
            .transparent(true)
            .decorations(false)
            .skip_taskbar(true)
            .shadow(false)
            .build()?;

    let screens = get_screen_info();
    let main_screen = screens
        .iter()
        .find(|screen| screen.is_main)
        .ok_or_else(|| anyhow::anyhow!("No main screen found"))?;
    let window_size = widget_window
        .inner_size()?
        .to_logical::<f64>(widget_window.scale_factor()?);

    widget_window.set_position(Position::Logical(LogicalPosition::new(
        main_screen.full.width as f64 - window_size.width,
        0.0,
    )))?;

    tokio::spawn(start_monitoring(app.handle().clone()));

    Ok(())
}
