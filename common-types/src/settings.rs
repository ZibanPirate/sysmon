use crate::screen::ScreenInfo;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct SettingsEvent {
    updated_settings: Settings,
}

impl SettingsEvent {
    pub fn new(updated_settings: Settings) -> Self {
        Self { updated_settings }
    }
}

// todo-zm: make typeshare play nicely with nest_struct (cargo-expand-only ?)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct Settings {
    pub network_widget: SettingsNetworkWidget,
    pub general: SettingsGeneral,
}

// widget
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct SettingsNetworkWidget {
    pub enabled: bool,
    pub position_per_screen_set: Vec<WidgetPositionForScreenIdSet>,
    pub safe_area: bool,
    pub size: f64,
    pub aspect_ratio: f64,
}

pub trait ListOfWidgetPositionForScreenIdSet {
    fn get_for_screen_set(
        &self,
        screen_set: &Vec<ScreenInfo>,
    ) -> Option<&WidgetPositionForScreenIdSet>;
    fn insert_new_screen_set(
        &mut self,
        screen_set: &Vec<ScreenInfo>,
    ) -> &WidgetPositionForScreenIdSet;
}

impl ListOfWidgetPositionForScreenIdSet for Vec<WidgetPositionForScreenIdSet> {
    fn get_for_screen_set(
        &self,
        screen_set: &Vec<ScreenInfo>,
    ) -> Option<&WidgetPositionForScreenIdSet> {
        let screen_id_set: Vec<String> =
            screen_set.iter().map(|screen| screen.id.clone()).collect();

        self.iter()
            .find(|widget| widget.screen_id_set == screen_id_set)
    }

    fn insert_new_screen_set(
        &mut self,
        screen_set: &Vec<ScreenInfo>,
    ) -> &WidgetPositionForScreenIdSet {
        let screen_id_set: Vec<String> =
            screen_set.iter().map(|screen| screen.id.clone()).collect();
        let main_screen = screen_set
            .iter()
            .find(|screen| screen.is_main)
            .or_else(|| screen_set.first())
            .expect("At least one screen should exist");
        let screen_id = main_screen.id.clone();

        let new_item = WidgetPositionForScreenIdSet {
            screen_id_set,
            screen_id,
            position: WidgetPosition::default(),
        };

        self.push(new_item);

        &self
            .last()
            .expect("Just pushed an item, so it should exist")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct WidgetPositionForScreenIdSet {
    pub screen_id_set: Vec<String>,
    pub screen_id: String,
    pub position: WidgetPosition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub enum WidgetPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for WidgetPosition {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        return WidgetPosition::TopRight;
        #[cfg(target_os = "windows")]
        return WidgetPosition::BottomRight;
    }
}
// ---

// general
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct SettingsGeneral {
    pub start_on_boot: bool,
    pub send_usage_telemetry: bool,
}
// ---

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: SettingsGeneral {
                #[cfg(not(debug_assertions))]
                start_on_boot: true,
                #[cfg(debug_assertions)]
                start_on_boot: false,
                // todo-zm: use setting send_usage_telemetry
                send_usage_telemetry: false,
            },
            network_widget: SettingsNetworkWidget {
                enabled: true,
                position_per_screen_set: vec![],
                #[cfg(target_os = "macos")]
                safe_area: false,
                #[cfg(target_os = "windows")]
                safe_area: true,
                size: 200.0,
                aspect_ratio: 3.0,
            },
        }
    }
}
