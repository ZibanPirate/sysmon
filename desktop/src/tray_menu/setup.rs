use crate::settings::window::show_settings_window;
use anyhow::{Ok, Result};
use tauri::menu::{MenuBuilder, MenuItemBuilder};

#[cfg(not(target_os = "macos"))]
use tauri::menu::AboutMetadataBuilder;

pub fn setup_tray_menu<'a>(app: &'a mut tauri::App) -> Result<()> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| anyhow::anyhow!("Tray with ID 'main' not found"))?;

    tray.set_menu(Some(
        MenuBuilder::new(app)
            // todo-zm: have custom About webview window
            .about(
                #[cfg(target_os = "macos")]
                None,
                #[cfg(not(target_os = "macos"))]
                Some(
                    AboutMetadataBuilder::new()
                        .name(Some("System Monitor"))
                        .version(Some(env!("CARGO_PKG_VERSION")))
                        .website(Some("https://sysmon.zak-man.com"))
                        .build(),
                ),
            )
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
