use serde::{Deserialize, Serialize};
use typeshare::typeshare;

// todo-zm: make typeshare play nicely with nest_struct (cargo-expand-only ?)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct Settings {
    pub tray: SettingsTray,
    pub network_widget: SettingsNetworkWidget,
    pub general: SettingsGeneral,
}

// tray
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct SettingsTray {
    pub content: SettingsTrayContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub enum SettingsTrayContent {
    Fixed,
    Network,
}
// ---

// widget
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct SettingsNetworkWidget {
    pub enabled: bool,
    pub position: SettingsNetworkWidgetPosition,
    pub safe_area: bool,
    pub size: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub enum SettingsNetworkWidgetPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
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
