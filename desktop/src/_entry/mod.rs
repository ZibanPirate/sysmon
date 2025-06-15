use crate::widget::setup::setup_widget;

pub fn app_builder() -> tauri::Builder<tauri::Wry> {
    tauri::Builder::default().setup(|app| {
        setup_widget(app)?;

        Ok(())
    })
}
