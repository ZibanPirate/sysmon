use serde::{Deserialize, Serialize};
use std::ops::Div;
use tauri::Position;
use typeshare::typeshare;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[typeshare]
pub struct CurrentScreenIdSetEvent {
    updated_current_screen_id_set: Vec<String>,
}

impl CurrentScreenIdSetEvent {
    pub fn new(updated_current_screen_id_set: Vec<String>) -> Self {
        Self {
            updated_current_screen_id_set,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[typeshare]
pub struct ScreenInfo {
    pub id: String,
    pub is_main: bool,
    pub full: Rect,
    pub safe: Rect,
}

impl ScreenInfo {
    pub fn new(id: String, is_main: bool, full: Rect, safe: Rect) -> Self {
        ScreenInfo {
            id,
            is_main,
            full,
            safe,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[typeshare]
pub struct Rect {
    #[typeshare(typescript(type = "number"))]
    pub x: i64,
    #[typeshare(typescript(type = "number"))]
    pub y: i64,
    #[typeshare(typescript(type = "number"))]
    pub width: i64,
    #[typeshare(typescript(type = "number"))]
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

impl Div<f64> for Rect {
    type Output = Rect;

    fn div(self, rhs: f64) -> Self::Output {
        Rect {
            x: (self.x as f64 / rhs) as i64,
            y: (self.y as f64 / rhs) as i64,
            width: (self.width as f64 / rhs) as i64,
            height: (self.height as f64 / rhs) as i64,
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
