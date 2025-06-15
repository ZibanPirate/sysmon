use std::os::raw::c_void;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
};

pub fn get_windows_desktop_work_area() -> Result<(i32, i32, i32, i32), Box<dyn std::error::Error>> {
    let mut rect = RECT::default();
    unsafe {
        let _result = SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut rect as *mut _ as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )?;
    }

    Ok((
        rect.left as i32,
        rect.top as i32,
        (rect.right - rect.left) as i32,
        (rect.bottom - rect.top) as i32,
    ))
}
