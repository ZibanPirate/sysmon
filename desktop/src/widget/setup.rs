use crate::{
    settings::command::observe_settings,
    widget::{
        monitor::start_monitoring,
        window::{RefreshWidgetTrigger, refresh_widget},
    },
};
use anyhow::Result;

#[cfg(target_os = "windows")]
use lib_cpp::observe_screen_info;
#[cfg(target_os = "macos")]
use lib_swift::observe_screen_info;

pub fn setup_widget<'a>(app: &'a mut tauri::App) -> Result<()> {
    refresh_widget(&app.handle(), RefreshWidgetTrigger::Initial)?;

    let app_handle = app.handle().clone();

    #[cfg(target_os = "macos")]
    observe_screen_info(move || refresh_widget(&app_handle, RefreshWidgetTrigger::ScreenChange));
    #[cfg(target_os = "windows")]
    observe_screen_info(move || refresh_widget(&app_handle, RefreshWidgetTrigger::ScreenChange));

    observe_settings(|app_handle| {
        refresh_widget(&app_handle, RefreshWidgetTrigger::SettingsChange)?;
        Ok(())
    });

    tokio::spawn(start_monitoring(app.handle().clone()));

    Ok(())
}
