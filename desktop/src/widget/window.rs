use anyhow::Result;
use common_types::{
    screen::CurrentScreenIdSetEvent,
    settings::{ListOfWidgetPositionForScreenIdSet, Settings, WidgetPosition},
};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, WebviewWindowBuilder};

#[derive(Debug, PartialEq, Eq)]
pub enum RefreshWidgetTrigger {
    Initial,
    SettingsChange,
    ScreenChange,
}

// todo-zm: atomic debounce this function
pub fn refresh_widget(app_handle: &AppHandle, from: RefreshWidgetTrigger) -> Result<()> {
    let settings_lock = app_handle.state::<Mutex<Settings>>();
    let settings = &mut settings_lock.lock().unwrap().network_widget;

    if !settings.enabled {
        if let Some(widget_window) = app_handle.get_webview_window("widget") {
            widget_window.close()?;
        }
        return Ok(());
    }

    let window_width = settings.size;
    let window_height = window_width / settings.aspect_ratio;
    let size = LogicalSize::new(window_width, window_height);

    #[cfg(target_os = "macos")]
    let screen_set = lib_swift::get_screen_info();
    #[cfg(target_os = "windows")]
    let screen_set = lib_cpp::get_screen_info();

    let widget_position_for_screen_id_set = if let Some(found) = settings
        .position_per_screen_set
        .get_for_screen_set(&screen_set)
    {
        found
    } else {
        settings
            .position_per_screen_set
            .insert_new_screen_set(&screen_set)
    };

    let selected_screen = screen_set
        .iter()
        .find(|screen| screen.id == widget_position_for_screen_id_set.screen_id)
        .ok_or(anyhow::anyhow!("At least one screen should exist"))?;

    if from == RefreshWidgetTrigger::ScreenChange {
        app_handle.emit(
            "current_screen_id_set_changed",
            CurrentScreenIdSetEvent::new(widget_position_for_screen_id_set.screen_id_set.clone()),
        )?;
    }

    let screen_rect = match settings.safe_area {
        true => &selected_screen.safe,
        false => &selected_screen.full,
    };

    let position = match widget_position_for_screen_id_set.position {
        WidgetPosition::TopLeft => LogicalPosition::new(screen_rect.x as f64, screen_rect.y as f64),
        WidgetPosition::TopRight => LogicalPosition::new(
            (screen_rect.width + screen_rect.x) as f64 - size.width,
            screen_rect.y as f64,
        ),
        WidgetPosition::BottomLeft => LogicalPosition::new(
            screen_rect.x as f64,
            (screen_rect.height + screen_rect.y) as f64 - size.height,
        ),
        WidgetPosition::BottomRight => LogicalPosition::new(
            (screen_rect.width + screen_rect.x) as f64 - size.width,
            (screen_rect.height + screen_rect.y) as f64 - size.height,
        ),
    };

    let Some(window) = app_handle.get_webview_window("widget") else {
        let app_handle = app_handle.clone();
        // note: on Windows OS, we must create new windows in a separate thread
        std::thread::spawn(move || {
            WebviewWindowBuilder::new(
                &app_handle,
                "widget",
                tauri::WebviewUrl::App("src/_entries/widget.html".into()),
            )
            .always_on_top(true)
            .resizable(false)
            .transparent(true)
            .decorations(false)
            .skip_taskbar(true)
            .shadow(false)
            .focused(false)
            // settings props on showing hidden window
            .inner_size(size.width, size.height)
            .position(position.x, position.y)
            // ---
            .build()
            .expect("Failed to create widget window")
            .set_ignore_cursor_events(true)
            .expect("Failed to set ignore cursor events");
        });

        return Ok(());
    };

    // settings props to update already visible window
    window.set_size(size)?;
    window.set_position(position)?;
    // ---

    Ok(())
}
