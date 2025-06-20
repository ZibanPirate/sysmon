use serde::{Deserialize, Serialize};
use std::time::Instant;
use tauri::Position;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum MonitorEvent {
    // CPU { percentage: f64 }, // todo-zm: implement CPU monitoring
    Network { sent: f64, received: f64 },
}

#[derive(Debug, Clone)]
pub struct ScreenInfo {
    pub full: Rect,
    pub safe: Rect,
    pub is_main: bool,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
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
    pub total_sent: isize,
    pub total_received: isize,
    pub timestamp: Instant,
}
