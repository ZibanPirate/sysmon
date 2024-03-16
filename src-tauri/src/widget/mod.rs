use std::sync::Arc;

use crate::settings::{Settings, SettingsPath};
use crate::utils::StateSubscriber;
use crate::Store;
use tauri::{App, Manager, WebviewWindowBuilder};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn setup_widget(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.state::<Store>();
    let mut settings_state = store
        .inner()
        .settings
        .lock()
        .map_err(|_| "Failed to lock settings")?;
    let state = settings_state.get_state();

    let widget_window_builder =
        WebviewWindowBuilder::new(app, "widget", tauri::WebviewUrl::App("index.html".into()));

    let widget_window_builder = widget_window_builder
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .inner_size(200.0, 50.0)
        .shadow(false)
        .visible(state.show_widget)
        .title_bar_style(tauri::TitleBarStyle::Transparent);

    let widget_window = widget_window_builder
        .build()
        .map_err(|err| format!("Failed to build widget window: {}", err))?;

    let arced_widget_window = Arc::new(widget_window.as_ref().window());

    settings_state.set_state(Settings {
        // @TODO-ZM: do we have to use Arc for `settings_state.widget_window`?
        widget_window: Some(arced_widget_window.clone()),
        ..state
    });

    widget_window
        .set_ignore_cursor_events(true)
        .map_err(|err| format!("Failed to set ignore cursor events: {}", err))?;

    widget_window
        .as_ref()
        .window()
        .move_window(Position::TopRight)
        .map_err(|err| format!("Failed to move window: {}", err))?;

    settings_state.on_path_change(
        SettingsPath::ShowWidget,
        Box::new(|new_state: &Settings| {
            let widget_window = new_state
                .widget_window
                .as_ref()
                .expect("Failed to get widget window");
            match new_state.show_widget {
                true => widget_window.show().expect("Failed to show widget window"),
                false => widget_window.hide().expect("Failed to hide widget window"),
            }
        }),
    );

    Ok(())
}
