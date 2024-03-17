use crate::utils::StateSubscriber;
use derivative::Derivative;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use tauri_plugin_positioner::Position;

// @TODO-ZM: add clone to tauri_plugin_positioner
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WidgetPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Into<Position> for WidgetPosition {
    fn into(self) -> Position {
        match self {
            WidgetPosition::TopRight => Position::TopRight,
            WidgetPosition::TopLeft => Position::TopLeft,
            WidgetPosition::BottomRight => Position::BottomRight,
            WidgetPosition::BottomLeft => Position::BottomLeft,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub show_widget: bool,
    pub widget_position: WidgetPosition,
    #[serde(skip)]
    pub widget_window: Option<Arc<tauri::Window>>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SettingsState {
    #[derivative(Debug = "ignore")]
    subscribers: HashMap<u32, Vec<(Vec<SettingsPath>, Box<dyn Fn(&Settings) -> () + Send>)>>,
    show_widget: bool,
    widget_position: WidgetPosition,
    widget_window: Option<Arc<tauri::Window>>,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            subscribers: HashMap::new(),
            show_widget: false,
            widget_position: WidgetPosition::TopRight,
            widget_window: None,
        }
    }
}

impl SettingsState {
    pub fn into_state(&self) -> Settings {
        Settings {
            show_widget: self.show_widget.clone(),
            widget_position: self.widget_position.clone(),
            widget_window: self.widget_window.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsPath {
    ShowWidget,
    WidgetPosition,
}

impl StateSubscriber<Settings, SettingsState, SettingsPath> for SettingsState {
    fn on_path_change(
        &mut self,
        changed_path: SettingsPath,
        callback: Box<dyn Fn(&Settings) -> () + Send>,
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
        callback: Box<dyn Fn(&Settings) -> () + Send>,
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
