use crate::settings::{Settings, SettingsPath, WidgetPosition};
use crate::utils::StateSubscriber;
use crate::Store;
use std::sync::Arc;
use tauri::{App, Manager, PhysicalPosition, PhysicalSize, Runtime, WebviewWindowBuilder, Window};

pub trait WidgetWindow {
    fn move_widget(
        &self,
        position: &WidgetPosition,
        safe_area: bool,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl<R: Runtime> WidgetWindow for Window<R> {
    fn move_widget(
        &self,
        position: &WidgetPosition,
        safe_area: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let screen = self.current_monitor()?.unwrap();
        let (screen_size, screen_position) = match safe_area {
            true => {
                let app = self.app_handle();

                let maximized_window = match app.get_webview_window("maximized_window") {
                    Some(window) => window,
                    None => {
                        let maximized_window_builder = WebviewWindowBuilder::new(
                            app,
                            "maximized_window",
                            tauri::WebviewUrl::App("maximized.html".into()),
                        );

                        let maximized_window_builder = maximized_window_builder
                            .decorations(false)
                            .maximized(true)
                            .transparent(true)
                            .skip_taskbar(true)
                            .shadow(false);

                        let maximized_window = maximized_window_builder
                            .build()
                            .map_err(|err| format!("Failed to build widget window: {}", err))?;

                        maximized_window.set_ignore_cursor_events(true)?;

                        maximized_window
                    }
                };

                let size = PhysicalSize::<i32> {
                    width: maximized_window.outer_size()?.width as i32,
                    height: maximized_window.outer_size()?.height as i32,
                };

                let max_pos: PhysicalPosition<i32> = maximized_window.outer_position()?;

                (size, max_pos)
            }
            false => (
                PhysicalSize::<i32> {
                    width: screen.size().width as i32,
                    height: screen.size().height as i32,
                },
                screen.position().clone(),
            ),
        };

        let window_size = PhysicalSize::<i32> {
            width: self.outer_size()?.width as i32,
            height: self.outer_size()?.height as i32,
        };

        let physical_position = match position {
            WidgetPosition::TopLeft => screen_position,
            WidgetPosition::TopRight => PhysicalPosition {
                x: screen_position.x + (screen_size.width - window_size.width),
                y: screen_position.y,
            },
            WidgetPosition::BottomLeft => PhysicalPosition {
                x: screen_position.x,
                y: screen_size.height - (window_size.height - screen_position.y),
            },
            WidgetPosition::BottomRight => PhysicalPosition {
                x: screen_position.x + (screen_size.width - window_size.width),
                y: screen_size.height - (window_size.height - screen_position.y),
            },
        };

        self.set_position(tauri::Position::Physical(physical_position))?;
        Ok(())
    }
}

pub fn setup_widget(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.state::<Store>();
    let mut settings_state = store
        .inner()
        .settings
        .lock()
        .map_err(|_| "Failed to lock settings")?;
    let state = settings_state.get_state();

    let widget_window_builder =
        WebviewWindowBuilder::new(app, "widget", tauri::WebviewUrl::App("index.html".into()));

    let widget_window_builder = widget_window_builder
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .inner_size(200.0, 50.0)
        .shadow(false)
        .visible(state.show_widget);

    let widget_window = widget_window_builder
        .build()
        .map_err(|err| format!("Failed to build widget window: {}", err))?;

    let arced_widget_window = Arc::new(widget_window.as_ref().window());

    settings_state.set_state(Settings {
        // @TODO-ZM: do we have to use Arc for `settings_state.widget_window`?
        widget_window: Some(arced_widget_window.clone()),
        ..state
    });

    widget_window
        .set_ignore_cursor_events(true)
        .map_err(|err| format!("Failed to set ignore cursor events: {}", err))?;

    settings_state.on_path_change(
        SettingsPath::ShowWidget,
        Box::pin(|new_state: &Settings| {
            let widget_window = new_state
                .widget_window
                .as_ref()
                .expect("Failed to get widget window");
            match new_state.show_widget {
                true => widget_window.show().expect("Failed to show widget window"),
                false => widget_window.hide().expect("Failed to hide widget window"),
            }
        }),
    );

    let move_window = Box::pin(|new_state: &Settings| {
        let widget_window = new_state
            .widget_window
            .as_ref()
            .expect("Failed to get widget window");
        widget_window
            .move_widget(&new_state.widget_position, new_state.safe_area)
            .expect("Failed to move widget window");
    });
    move_window(&settings_state.get_state());
    settings_state.on_paths_change(
        vec![SettingsPath::WidgetPosition, SettingsPath::SafeArea],
        move_window,
    );

    let emit_settings_to_widget_window = Box::pin(move |new_state: &Settings| {
        let widget_window = new_state
            .widget_window
            .as_ref()
            .expect("Failed to get widget window");
        widget_window
            .emit("settings", new_state)
            .expect("Failed to emit settings");
    });
    println!("state is {:?}", settings_state.get_state());
    emit_settings_to_widget_window(&settings_state.get_state());
    settings_state.on_paths_change(vec![], emit_settings_to_widget_window);

    Ok(())
}
