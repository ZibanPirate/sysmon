use anyhow::Result;
use common_types::screen::{Rect, ScreenInfo};
use std::sync::{LazyLock, Mutex};

#[cxx::bridge]
mod ffi {

    #[derive(Debug, PartialEq, Eq, Clone)]
    enum CppMessage {
        ScreenInfoChanged,
    }

    extern "Rust" {
        type CRect;
        fn new_boxed_rect(x: i64, y: i64, width: i64, height: i64) -> Box<CRect>;

        type ScreenInfoVec;
        fn new_boxed_screen_info_vec() -> Box<ScreenInfoVec>;
        fn push_new_screen_info(
            self: &mut ScreenInfoVec,
            is_main: bool,
            scale_factor: f64,
            full: Box<CRect>,
            safe: Box<CRect>,
        );

        fn message_from_cpp(message: CppMessage);
    }

    unsafe extern "C++" {
        include!("crate-root/cpp/src/lib.h");

        fn get_screen_info() -> Box<ScreenInfoVec>;

        fn start_observing_screen_info();
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
    pub fn push_new_screen_info(
        &mut self,
        is_main: bool,
        scale_factor: f64,
        full: Box<CRect>,
        safe: Box<CRect>,
    ) {
        self.screens.push(ScreenInfo::new(
            is_main,
            full.0 / scale_factor,
            safe.0 / scale_factor,
        ));
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

pub fn observe_screen_info<T: Fn() -> Result<()> + Send + 'static>(callback: T) {
    MESSAGE_CALLBACKS
        .lock()
        .unwrap()
        .push((ffi::CppMessage::ScreenInfoChanged, Box::new(callback)));

    // starts observing if not already started
    ffi::start_observing_screen_info();
}

static MESSAGE_CALLBACKS: LazyLock<
    Mutex<Vec<(ffi::CppMessage, Box<dyn Fn() -> Result<()> + Send>)>>,
> = LazyLock::new(|| Mutex::new(Vec::new()));

fn message_from_cpp(message: ffi::CppMessage) {
    let callbacks = MESSAGE_CALLBACKS.lock().unwrap();
    for (msg, callback) in callbacks.iter() {
        if *msg == message {
            let Ok(_) = callback() else {
                // todo-zm: report error
                eprintln!("Error executing callback for message: {:?}", message);
                continue;
            };
        }
    }
}

// todo-zm: add proper tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_result() {
        ffi::start_observing_screen_info();
        // let result = get_screen_info();
        // let result = format!("Result from Cpp: {:?}", result);
        // assert_eq!(result, "");
    }
}
