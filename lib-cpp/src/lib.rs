use common_types::screen::{Rect, ScreenInfo};

#[cxx::bridge]
mod ffi {

    extern "Rust" {
        type CRect;
        fn new_boxed_rect(x: i64, y: i64, width: i64, height: i64) -> Box<CRect>;

        type ScreenInfoVec;
        fn new_boxed_screen_info_vec() -> Box<ScreenInfoVec>;
        fn push_new_screen_info(
            self: &mut ScreenInfoVec,
            is_main: bool,
            full: Box<CRect>,
            safe: Box<CRect>,
        );
    }

    unsafe extern "C++" {
        include!("crate-root/cpp/src/lib.h");

        fn get_screen_info() -> Box<ScreenInfoVec>;
    }
}

#[derive(Debug)]
pub struct CRect(pub Rect);

fn new_boxed_rect(x: i64, y: i64, width: i64, height: i64) -> Box<CRect> {
    Box::new(CRect(Rect::new(x, y, width, height)))
}

#[derive(Debug)]
struct ScreenInfoVec {
    screens: Vec<ScreenInfo>,
}

impl ScreenInfoVec {
    pub fn push_new_screen_info(&mut self, is_main: bool, full: Box<CRect>, safe: Box<CRect>) {
        self.screens.push(ScreenInfo::new(is_main, full.0, safe.0));
    }
}

fn new_boxed_screen_info_vec() -> Box<ScreenInfoVec> {
    Box::new(ScreenInfoVec {
        screens: Vec::new(),
    })
}

pub fn get_screen_info() -> Vec<ScreenInfo> {
    ffi::get_screen_info().screens
}

// todo-zm: add proper tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_result() {
        let result = get_screen_info();
        let result = format!("Result from Cpp: {:?}", result);
        assert_eq!(result, "Result from Cpp:");
    }
}
