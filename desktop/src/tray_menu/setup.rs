use crate::settings::window::show_settings_window;
use anyhow::{Ok, Result};
use tauri::menu::{MenuBuilder, MenuItemBuilder};

pub fn setup_tray_menu<'a>(app: &'a mut tauri::App) -> Result<()> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| anyhow::anyhow!("Tray with ID 'main' not found"))?;

    tray.set_menu(Some(
        MenuBuilder::new(app)
            // todo-zm: about don't work on Windows
            .about(None)
            .item(&MenuItemBuilder::new("Settings").id("settings").build(app)?)
            .separator()
            .quit()
            .build()?,
    ))?;

    tray.on_menu_event(|app, event| {
        match event {
            tauri::menu::MenuEvent { id, .. } => {
                if id == "settings" {
                    if let Err(e) = show_settings_window(app) {
                        // todo-zm: report-error
                        eprintln!("Error opening settings window: {}", e);
                    }
                }
            }
        }
    });

    Ok(())
}
