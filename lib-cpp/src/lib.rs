use anyhow::Result;
use common_types::network::NetworkInfo;
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
            id: String,
            is_main: bool,
            full: Box<CRect>,
            safe: Box<CRect>,
        );

        fn message_from_cpp(message: CppMessage);

        type CNetworkInfo;
        fn new_boxed_network_info(total_sent: u64, total_received: u64) -> Box<CNetworkInfo>;
    }

    unsafe extern "C++" {
        include!("crate-root/cpp/src/lib.h");

        fn get_screen_info() -> Box<ScreenInfoVec>;

        fn start_observing_screen_info();

        fn get_network_info() -> Box<CNetworkInfo>;
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
        id: String,
        is_main: bool,
        full: Box<CRect>,
        safe: Box<CRect>,
    ) {
        self.screens
            .push(ScreenInfo::new(id, is_main, full.0, safe.0));
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
                // todo-zm: report-error
                eprintln!("Error executing callback for message: {:?}", message);
                continue;
            };
        }
    }
}

struct CNetworkInfo(pub NetworkInfo);

fn new_boxed_network_info(total_sent: u64, total_received: u64) -> Box<CNetworkInfo> {
    Box::new(CNetworkInfo(NetworkInfo::new(total_sent, total_received)))
}

pub fn get_network_info() -> NetworkInfo {
    ffi::get_network_info().0
}

// todo-zm: add proper tests
#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn screen_info_should_be_non_empty_array() {
        let result = get_screen_info();
        assert!(!result.is_empty(), "Screen info should not be empty");
    }

    #[test]
    fn network_info_should_be_more_than_zero_bytes() {
        let result = get_network_info();
        assert!(
            result.total_sent > 0 || result.total_received > 0,
            "Network info should not be zero"
        );
    }
}
