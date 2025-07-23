use anyhow::Result;
use common_types::{
    network::NetworkInfo,
    screen::{Rect, ScreenInfo},
};
use std::sync::{LazyLock, Mutex};

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Rect;
        #[swift_bridge(associated_to = Rect)]
        fn new(x: i64, y: i64, width: i64, height: i64) -> Rect;

        type ScreenInfo;
        #[swift_bridge(associated_to = ScreenInfo)]
        fn new(isMain: bool, full: Rect, safe: Rect) -> ScreenInfo;

        type NetworkInfo;
        #[swift_bridge(associated_to = NetworkInfo)]
        fn new(total_sent: u64, total_received: u64) -> NetworkInfo;
    }

    extern "Rust" {
        type SwiftMessage;
        #[swift_bridge(associated_to = SwiftMessage)]
        fn new_screen_info_changed() -> SwiftMessage;

        #[swift_bridge(swift_name = "messageFromSwift")]
        fn message_from_swift(message: SwiftMessage);
    }

    extern "Swift" {
        #[swift_bridge(swift_name = "getScreenInfo")]
        fn get_screen_info() -> Vec<ScreenInfo>;

        #[swift_bridge(swift_name = "getNetworkInfo")]
        fn get_network_info_vec() -> Vec<NetworkInfo>;

        #[swift_bridge(swift_name = "startObservingScreenInfo")]
        fn start_observing_screen_info();
    }
}

pub fn get_screen_info() -> Vec<ScreenInfo> {
    ffi::get_screen_info()
}

pub fn get_network_info() -> NetworkInfo {
    ffi::get_network_info_vec().first().unwrap().to_owned()
}

pub fn observe_screen_info<T: Fn() -> Result<()> + Send + 'static>(callback: T) {
    MESSAGE_CALLBACKS
        .lock()
        .unwrap()
        .push((SwiftMessage::ScreenInfoChanged, Box::new(callback)));

    // starts observing if not already started
    ffi::start_observing_screen_info();
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SwiftMessage {
    ScreenInfoChanged,
}
impl SwiftMessage {
    fn new_screen_info_changed() -> Self {
        SwiftMessage::ScreenInfoChanged
    }
}

static MESSAGE_CALLBACKS: LazyLock<Mutex<Vec<(SwiftMessage, Box<dyn Fn() -> Result<()> + Send>)>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

fn message_from_swift(message: SwiftMessage) {
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

// todo-zm: add proper tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_result() {
        let result = get_network_info();
        let result = format!("Result from Swift: {:?}", result);
        assert_eq!(result, "Result from Swift:");
    }
}
