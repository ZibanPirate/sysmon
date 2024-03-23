// use winit::{
//     event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
//     event_loop::EventLoop,
//     platform::windows::{WindowExtWindows, HWND},
//     window::{WindowBuilder, WindowLevel},
// };

use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoop,
    window::{WindowBuilder, WindowLevel},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_visible(true)
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();
    window.set_window_level(WindowLevel::AlwaysOnTop);
    window.set_cursor_hittest(false).unwrap();

    event_loop.run(move |event, elwt| {
        //
    });
}
