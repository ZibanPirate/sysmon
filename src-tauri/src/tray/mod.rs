use crate::settings::WidgetPosition;
use crate::Store;
use crate::{settings::Settings, utils::StateSubscriber};
use tauri::menu::MenuItemBuilder;
use tauri::AppHandle;
use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuEvent, SubmenuBuilder},
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

    let tray_menu = |state: &Settings, app: &AppHandle| {
        MenuBuilder::new(app)
            .items(&[
                &CheckMenuItemBuilder::new("Show Widget")
                    .id("show-widget")
                    .checked(state.show_widget)
                    .build(app)
                    .unwrap(),
                &MenuItemBuilder::new("Refresh Widget")
                    .id("refresh-widget")
                    .build(app)
                    .unwrap(),
                &SubmenuBuilder::new(app, "Position")
                    .items(&[
                        &CheckMenuItemBuilder::new(" ⌝ Top-Right")
                            .id("top-right")
                            .checked(state.widget_position == WidgetPosition::TopRight)
                            .build(app)
                            .unwrap(),
                        &CheckMenuItemBuilder::new(" ⌜ Top-Left")
                            .id("top-left")
                            .checked(state.widget_position == WidgetPosition::TopLeft)
                            .build(app)
                            .unwrap(),
                        &CheckMenuItemBuilder::new(" ⌟ Bottom-Right")
                            .id("bottom-right")
                            .checked(state.widget_position == WidgetPosition::BottomRight)
                            .build(app)
                            .unwrap(),
                        &CheckMenuItemBuilder::new(" ⌞ Bottom-Left")
                            .id("bottom-left")
                            .checked(state.widget_position == WidgetPosition::BottomLeft)
                            .build(app)
                            .unwrap(),
                    ])
                    .separator()
                    .items(&[&CheckMenuItemBuilder::new("Account for OS Taskbar/Dock")
                        .id("safe-area")
                        .checked(state.safe_area)
                        .build(app)
                        .unwrap()])
                    .build()
                    .unwrap(),
            ])
            .separator()
            .quit()
            .separator()
            .item(
                &MenuItemBuilder::new(format!("Sysmon v{}", env!("CARGO_PKG_VERSION")).as_str())
                    .build(app)
                    .unwrap(),
            )
            .build()
            .unwrap()
    };

    let tray = app.tray().unwrap();

    tray.set_menu(Some(tray_menu(&state, app.app_handle())))
        .map_err(|err| format!("Failed to set tray menu: {}", err))
        .unwrap();

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
                "refresh-widget" => {
                    settings_state.set_state(Settings {
                        last_manually_refreshed: Some(chrono::Utc::now().timestamp()),
                        ..state
                    });
                }
                "top-right" => {
                    settings_state.set_state(Settings {
                        widget_position: WidgetPosition::TopRight,
                        ..state
                    });
                }
                "top-left" => {
                    settings_state.set_state(Settings {
                        widget_position: WidgetPosition::TopLeft,
                        ..state
                    });
                }
                "bottom-right" => {
                    settings_state.set_state(Settings {
                        widget_position: WidgetPosition::BottomRight,
                        ..state
                    });
                }
                "bottom-left" => {
                    settings_state.set_state(Settings {
                        widget_position: WidgetPosition::BottomLeft,
                        ..state
                    });
                }
                "safe-area" => {
                    settings_state.set_state(Settings {
                        safe_area: !state.safe_area,
                        ..state
                    });
                }
                _ => {
                    println!("unhandled menu event: {:?}", id);
                }
            },
        }

        let state = settings_state.get_state();

        let tray_menu = tray_menu(&state, app);

        app.tray()
            .unwrap()
            .set_menu(Some(tray_menu))
            .map_err(|err| format!("Failed to set tray menu: {}", err))
            .unwrap();
    });

    Ok(())
}
