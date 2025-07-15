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
    pub tray: SettingsTray,
    pub network_widget: SettingsNetworkWidget,
    pub general: SettingsGeneral,
}

// tray
#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
// todo-zm: implement tray content settings
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
    pub size: f64,
    pub aspect_ratio: f64,
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
    // todo-zm: impl send_usage_telemetry setting
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
                // toto-zm: use setting send_usage_telemetry
                send_usage_telemetry: false,
            },
            network_widget: SettingsNetworkWidget {
                enabled: true,
                #[cfg(target_os = "macos")]
                position: SettingsNetworkWidgetPosition::TopRight,
                #[cfg(target_os = "windows")]
                position: SettingsNetworkWidgetPosition::BottomRight,
                #[cfg(target_os = "macos")]
                safe_area: false,
                #[cfg(target_os = "windows")]
                safe_area: true,
                size: 200.0,
                aspect_ratio: 3.0,
            },
            tray: SettingsTray {
                // toto-zm: use setting tray.content
                content: SettingsTrayContent::Network,
            },
        }
    }
}
