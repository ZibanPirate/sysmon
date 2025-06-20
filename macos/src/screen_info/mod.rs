use common_types::Rect;
use common_types::ScreenInfo;
use swift_rs::SRObjectArray;
use swift_rs::swift;

#[repr(C)]
#[derive(Debug, Clone)]
struct CScreenInfo {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    is_main: bool,
    safe_x: isize,
    safe_y: isize,
    safe_width: isize,
    safe_height: isize,
}

impl Into<ScreenInfo> for &CScreenInfo {
    fn into(self) -> ScreenInfo {
        ScreenInfo {
            full: Rect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: self.height,
            },
            safe: Rect {
                x: self.safe_x,
                y: self.safe_y,
                width: self.safe_width,
                height: self.safe_height,
            },
            is_main: self.is_main,
        }
    }
}

swift!(fn desktop_info() -> SRObjectArray<CScreenInfo>);

pub fn get_all_screen_info() -> Vec<ScreenInfo> {
    let result = unsafe { desktop_info() };
    let mut info = Vec::new();
    for screen in result.iter() {
        info.push(screen.as_ref().into());
    }
    info
}

// todo-zm: add tests
