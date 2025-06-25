use anyhow::Result;
use common_types::settings::{Settings, SettingsNetworkWidgetPosition};
use std::sync::Mutex;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, WebviewWindowBuilder};

#[cfg(target_os = "windows")]
use lib_cpp::get_screen_info;
#[cfg(target_os = "macos")]
use lib_swift::{get_screen_info, observe_screen_info};

// todo-zm: atomic debounce this function
pub fn refresh_widget(app_handle: &AppHandle) -> Result<()> {
    let settings_lock = app_handle.state::<Mutex<Settings>>();
    let settings = &settings_lock.lock().unwrap().network_widget;

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
    let screens = get_screen_info();
    #[cfg(target_os = "windows")]
    let screens = get_screen_info();

    let screen_rect = {
        let main_screen = screens
            .iter()
            .find(|screen| screen.is_main)
            .ok_or_else(|| anyhow::anyhow!("No main screen found"))?;
        match settings.safe_area {
            true => &main_screen.safe,
            false => &main_screen.full,
        }
    };

    let position = match settings.position {
        SettingsNetworkWidgetPosition::TopLeft => {
            LogicalPosition::new(screen_rect.x as f64, screen_rect.y as f64)
        }
        SettingsNetworkWidgetPosition::TopRight => LogicalPosition::new(
            (screen_rect.width + screen_rect.x) as f64 - size.width,
            screen_rect.y as f64,
        ),
        SettingsNetworkWidgetPosition::BottomLeft => LogicalPosition::new(
            screen_rect.x as f64,
            (screen_rect.height + screen_rect.y) as f64 - size.height,
        ),
        SettingsNetworkWidgetPosition::BottomRight => LogicalPosition::new(
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
