use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder},
    App, Manager,
};

use crate::Store;
use crate::{settings::Settings, utils::StateSubscriber};

pub fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.state::<Store>();
    let lock = store
        .inner()
        .settings
        .lock()
        .map_err(|_| "Failed to lock settings")?;
    let state = lock.get_state();

    let tray = app.tray().ok_or("Failed to create tray")?;

    tray.set_menu(Some(
        MenuBuilder::new(app)
            .items(&[
                &CheckMenuItemBuilder::new("Show Widget")
                    .id("show-widget")
                    .checked(state.show_widget)
                    .build(app)
                    .unwrap(),
                &SubmenuBuilder::new(app, "Position")
                    .items(&[
                        &MenuItemBuilder::new("Top-Right")
                            .enabled(false)
                            .build(app)
                            .unwrap(),
                        &MenuItemBuilder::new("Top-Left")
                            .enabled(false)
                            .build(app)
                            .unwrap(),
                        &MenuItemBuilder::new("Bottom-Right")
                            .enabled(false)
                            .build(app)
                            .unwrap(),
                        &MenuItemBuilder::new("Bottom-Left")
                            .enabled(false)
                            .build(app)
                            .unwrap(),
                    ])
                    .build()
                    .unwrap(),
            ])
            .separator()
            .quit()
            .separator()
            .about(None)
            .build()?,
    ))?;

    tray.on_menu_event(move |app, event| {
        let store = app.state::<Store>();
        let mut lock = store.inner().settings.lock().unwrap();
        let state = lock.get_state();

        match event {
            MenuEvent { id, .. } => match id.as_ref() {
                "show-widget" => {
                    lock.set_state(Settings {
                        show_widget: !state.show_widget,
                        ..state
                    });
                }
                _ => {}
            },
        }
    });

    Ok(())
}
