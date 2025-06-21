use tauri::Position;

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
