use crate::widget::setup::setup_widget;

pub fn app_builder() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            setup_widget(app)?;

            Ok(())
        })
}
