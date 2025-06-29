use crate::{
    auto_start::setup::setup_autostart, settings::setup::setup_settings_state,
    tray_menu::setup::setup_tray_menu, updater::setup::setup_updater, widget::setup::setup_widget,
};

pub fn app_builder() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory); // hide the dock icon

            setup_settings_state(app)?;
            setup_widget(app)?;
            setup_tray_menu(app)?;
            setup_autostart(app)?;
            setup_updater(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::settings::command::get_settings,
            crate::settings::command::set_settings
        ])
}
