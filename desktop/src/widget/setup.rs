use crate::{settings::command::observe_settings, widget::monitor::start_monitoring};
use anyhow::Result;
use common_types::settings::{Settings, SettingsNetworkWidgetPosition};
use lib_swift::{get_screen_info, observe_screen_info};
use std::sync::Mutex;
use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, WebviewWindowBuilder};

fn refresh_widget(app_handle: &AppHandle) -> Result<()> {
    let settings_lock = app_handle.state::<Mutex<Settings>>();
    let settings = &settings_lock.lock().unwrap().network_widget;

    if !settings.enabled {
        if let Some(widget_window) = app_handle.get_webview_window("widget") {
            widget_window.close()?;
        }
        return Ok(());
    }

    let window_width = settings.size;
    let window_height = window_width / 4.0;

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
            (screen_rect.width + screen_rect.x) as f64 - window_width,
            screen_rect.y as f64,
        ),
        SettingsNetworkWidgetPosition::BottomLeft => LogicalPosition::new(
            screen_rect.x as f64,
            (screen_rect.height + screen_rect.y) as f64 - window_height,
        ),
        SettingsNetworkWidgetPosition::BottomRight => LogicalPosition::new(
            (screen_rect.width + screen_rect.x) as f64 - window_width,
            (screen_rect.height + screen_rect.y) as f64 - window_height,
        ),
    };

    let Some(window) = app_handle.get_webview_window("widget") else {
        WebviewWindowBuilder::new(
            app_handle,
            "widget",
            tauri::WebviewUrl::App("src/_entries/widget.html".into()),
        )
        .always_on_top(true)
        .resizable(false)
        .transparent(true)
        .decorations(false)
        .skip_taskbar(true)
        .shadow(false)
        .accept_first_mouse(false)
        .focused(false)
        // settings props on showing hidden window
        .inner_size(window_width, window_height)
        .position(position.x, position.y)
        // ---
        .build()?;

        return Ok(());
    };

    // settings props to update already visible window
    window.set_size(LogicalSize::new(window_width, window_height))?;
    window.set_position(position)?;
    // ---

    Ok(())
}

pub fn setup_widget<'a>(app: &'a mut tauri::App) -> Result<()> {
    refresh_widget(&app.handle())?;

    let app_handle = app.handle().clone();

    observe_screen_info(move || refresh_widget(&app_handle));
    observe_settings(|app_handle| {
        refresh_widget(&app_handle)?;
        Ok(())
    });

    tokio::spawn(start_monitoring(app.handle().clone()));

    Ok(())
}
