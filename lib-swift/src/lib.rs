use common_types::{NetworkInfo, Rect, ScreenInfo};
pub use ffi::swift_multiply_by_4;

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
        fn new(total_sent: u32, total_received: u32) -> NetworkInfo;
    }

    extern "Rust" {
        fn rust_double_number(num: i64) -> i64;
    }

    extern "Swift" {
        fn swift_multiply_by_4(num: i64) -> i64;
        #[swift_bridge(swift_name = "getScreenInfo")]
        fn get_screen_info() -> Vec<ScreenInfo>;
        #[swift_bridge(swift_name = "getNetworkInfo")]
        fn get_network_info_vec() -> Vec<NetworkInfo>;
    }
}

pub fn get_screen_info() -> Vec<ScreenInfo> {
    ffi::get_screen_info()
}

pub fn get_network_info() -> NetworkInfo {
    ffi::get_network_info_vec().first().unwrap().to_owned()
}

// todo-zm: remove sup and rust_double_number
pub fn sup() -> String {
    // let result = ffi::swift_multiply_by_4(10);
    // let result = ffi::get_screen_info();
    let result = get_network_info();
    format!("Result from Swift: {:?}", result)
}

fn rust_double_number(num: i64) -> i64 {
    println!("Rust double function called...");

    num * 4
}

// todo-zm: add proper tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_result() {
        let result = sup();
        assert_eq!(result, "Result from Swift:");
    }
}
