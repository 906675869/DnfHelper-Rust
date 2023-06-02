use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;

use winapi::um::winuser::{MB_OK, MessageBoxW};

// 获取当前时间
pub fn get_now_date() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn sleep(t: u64) {
    thread::sleep(Duration::from_millis(t));
}

pub fn show_message_box(message: &str, caption: &str) {
    let wide_message: Vec<u16> = OsString::from(message)
        .encode_wide()
        .chain(Some(0))
        .collect();

    let wide_caption: Vec<u16> = OsString::from(caption)
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        MessageBoxW(
            null_mut(),
            wide_message.as_ptr(),
            wide_caption.as_ptr(),
            MB_OK,
        );
    }
}