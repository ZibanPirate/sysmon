use crate::Store;
use crate::{settings::Settings, utils::StateSubscriber};
use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuEvent, MenuItemBuilder, SubmenuBuilder},
    App, Manager,
};

pub fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.state::<Store>();
    let settings_state = store
        .inner()
        .settings
        .lock()
        .map_err(|_| "Failed to lock settings")?;
    let state = settings_state.get_state();

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
    ))
    .map_err(|err| format!("Failed to set tray menu: {}", err))?;

    tray.on_menu_event(move |app, event| {
        let store = app.state::<Store>();
        let mut settings_state = store
            .inner()
            .settings
            .lock()
            .expect("Failed to lock settings");
        let state = settings_state.get_state();

        match event {
            MenuEvent { id, .. } => match id.as_ref() {
                "show-widget" => {
                    settings_state.set_state(Settings {
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
