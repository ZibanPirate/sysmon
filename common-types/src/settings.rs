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

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: SettingsGeneral {
                start_on_boot: true,
                send_usage_telemetry: false,
            },
            network_widget: SettingsNetworkWidget {
                enabled: true,
                // todo-zm: change per OS
                position: SettingsNetworkWidgetPosition::TopLeft,
                // todo-zm: change per OS
                safe_area: false,
                size: 200.0,
            },
            tray: SettingsTray {
                content: SettingsTrayContent::Network,
            },
        }
    }
}
