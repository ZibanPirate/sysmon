use crate::_entry::app_builder;

mod _entry;
mod widget;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_builder = app_builder();

    app_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
