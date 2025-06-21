use crate::{tray_menu::setup::setup_tray_menu, widget::setup::setup_widget};

pub fn app_builder() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default().setup(|app| {
        setup_widget(app)?;
        setup_tray_menu(app)?;

        Ok(())
    })
}
