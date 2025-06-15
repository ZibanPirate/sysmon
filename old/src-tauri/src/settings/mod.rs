use crate::{utils::StateSubscriber, Store};
use derivative::Derivative;
use nest_struct::nest_struct;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::{create_dir_all, File},
    path::PathBuf,
    pin::Pin,
    sync::Arc,
};
use tauri::{App, Manager};

#[nest_struct]
#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub show_widget: bool,
    pub widget_position: nest! {
        // @TODO-ZM: add clone to tauri_plugin_positioner
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum WidgetPosition {
            TopRight,
            TopLeft,
            BottomRight,
            BottomLeft,
        }
    },
    pub safe_area: bool,
    pub last_manually_refreshed: Option<i64>,
    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    pub widget_window: Option<Arc<tauri::Window>>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SettingsState {
    #[derivative(Debug = "ignore")]
    subscribers: HashMap<u32, Vec<(Vec<SettingsPath>, Pin<Box<dyn Fn(&Settings) -> () + Send>>)>>,
    show_widget: bool,
    safe_area: bool,
    pub last_manually_refreshed: Option<i64>,
    widget_position: WidgetPosition,
    #[derivative(Debug = "ignore")]
    widget_window: Option<Arc<tauri::Window>>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            subscribers: HashMap::new(),
            show_widget: true,
            widget_position: WidgetPosition::TopRight,
            safe_area: true,
            last_manually_refreshed: None,
            widget_window: None,
        }
    }
}

impl SettingsState {
    pub fn into_state(&self) -> Settings {
        Settings {
            show_widget: self.show_widget,
            widget_position: self.widget_position.clone(),
            widget_window: self.widget_window.clone(),
            safe_area: self.safe_area,
            last_manually_refreshed: self.last_manually_refreshed,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsPath {
    ShowWidget,
    WidgetPosition,
    SafeArea,
    LastManuallyRefreshed,
}

impl StateSubscriber<Settings, SettingsState, SettingsPath> for SettingsState {
    fn on_path_change(
        &mut self,
        changed_path: SettingsPath,
        callback: Pin<Box<dyn Fn(&Settings) -> () + Send>>,
    ) -> u32 {
        let id = self.subscribers.len() as u32;
        self.subscribers
            .insert(id, vec![(vec![changed_path], callback)]);
        id
    }

    ///
    /// Pass empty vec to `changed_paths` to trigger callback on any state change
    ///     
    fn on_paths_change(
        &mut self,
        changed_paths: Vec<SettingsPath>,
        callback: Pin<Box<dyn Fn(&Settings) -> () + Send>>,
    ) -> u32 {
        let id = self.subscribers.len() as u32;
        self.subscribers.insert(id, vec![(changed_paths, callback)]);
        id
    }

    fn unsubscribe(&mut self, id: u32) {
        self.subscribers.remove(&id);
    }

    fn set_state(&mut self, new_state: Settings) {
        let mut changed_paths = vec![];
        if self.show_widget != new_state.show_widget {
            self.show_widget = new_state.show_widget.clone();
            changed_paths.push(SettingsPath::ShowWidget);
        }
        if self.widget_position != new_state.widget_position {
            self.widget_position = new_state.widget_position.clone();
            changed_paths.push(SettingsPath::WidgetPosition);
        }

        if self.widget_window != new_state.widget_window {
            self.widget_window = new_state.widget_window.clone();
        }

        if self.safe_area != new_state.safe_area {
            self.safe_area = new_state.safe_area.clone();
            changed_paths.push(SettingsPath::SafeArea);
        }

        if self.last_manually_refreshed != new_state.last_manually_refreshed {
            self.last_manually_refreshed = new_state.last_manually_refreshed.clone();
            changed_paths.push(SettingsPath::LastManuallyRefreshed);
        }

        for (_, subscribers) in self.subscribers.iter() {
            for (paths, callback) in subscribers.iter() {
                if paths.is_empty() || paths.iter().any(|path| changed_paths.contains(path)) {
                    callback(&new_state);
                }
            }
        }
    }

    fn get_state(&self) -> Settings {
        self.into_state()
    }
}

pub fn load_settings(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.state::<Store>();
    let mut settings_state = store
        .inner()
        .settings
        .lock()
        .map_err(|_| "Failed to lock settings")?;

    let app_config_path = app.path().app_config_dir()?;
    let settings_path = app_config_path.join("settings.json");

    match File::open(settings_path.clone()) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(settings) => settings_state.set_state(settings),
            Err(err) => println!("Failed to parse settings file: {}", err),
        },
        Err(err) => println!("Failed to open settings file: {}", err),
    };

    settings_state.on_paths_change(
        vec![],
        Box::pin(move |settings| {
            if save_settings(&app_config_path, &settings_path, settings.clone()).is_err() {
                println!("Failed to save settings");
            }
        }),
    );

    Ok(())
}

pub fn save_settings(
    app_config_path: &PathBuf,
    settings_path: &PathBuf,
    settings: Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(app_config_path)?;

    let file = File::create(settings_path)?;
    serde_json::to_writer_pretty(file, &settings)?;

    Ok(())
}
