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
    }

    extern "Rust" {
        fn rust_double_number(num: i64) -> i64;
    }

    extern "Swift" {
        fn swift_multiply_by_4(num: i64) -> i64;
        fn get_screen_info() -> Vec<ScreenInfo>;
    }
}

// todo-zm: move Rect and ScreenInfo to common-types crate
#[derive(Debug)]
struct Rect {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

impl Rect {
    fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug)]
struct ScreenInfo {
    is_main: bool,
    full: Rect,
    safe: Rect,
}

impl ScreenInfo {
    fn new(is_main: bool, full: Rect, safe: Rect) -> Self {
        ScreenInfo {
            is_main,
            full,
            safe,
        }
    }
}

// toodo-zm: remove sup and rust_double_number
pub fn sup() -> String {
    // let result = ffi::swift_multiply_by_4(10);
    let result = ffi::get_screen_info();
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
