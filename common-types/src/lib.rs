use serde::{Deserialize, Serialize};
use tauri::Position;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum MonitorEvent {
    // CPU { percentage: f64 }, // todo-zm: implement CPU monitoring
    Network { sent: f64, received: f64 },
}

impl MonitorEvent {
    pub fn new_network_from_tuple(speed: (u64, u64)) -> Self {
        MonitorEvent::Network {
            sent: speed.0 as f64,
            received: speed.1 as f64,
        }
    }
}

#[derive(Debug)]
pub struct ScreenInfo {
    pub is_main: bool,
    pub full: Rect,
    pub safe: Rect,
}

impl ScreenInfo {
    pub fn new(is_main: bool, full: Rect, safe: Rect) -> Self {
        ScreenInfo {
            is_main,
            full,
            safe,
        }
    }
}

#[derive(Debug)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

impl Rect {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

impl Into<Position> for &Rect {
    fn into(self) -> Position {
        Position::Physical(tauri::PhysicalPosition {
            x: self.x as i32,
            y: self.y as i32,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub total_sent: u32,
    pub total_received: u32,
}
impl NetworkInfo {
    pub fn new(total_sent: u32, total_received: u32) -> NetworkInfo {
        NetworkInfo {
            total_sent,
            total_received,
        }
    }
}
