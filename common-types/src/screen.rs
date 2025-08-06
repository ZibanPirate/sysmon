use std::ops::Div;
use tauri::Position;

#[derive(Debug)]
pub struct ScreenInfo {
    pub id: String,
    // todo-zm: remove is_main
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
