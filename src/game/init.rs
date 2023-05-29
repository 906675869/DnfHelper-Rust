use std::ffi::c_int;
use std::ptr::null_mut;

use winapi::um::winuser::{
    DispatchMessageW,
    GetMessageW,
    MOD_CONTROL,
    MSG,
    RegisterHotKey,
    TranslateMessage,
    VK_DOWN, VK_END, VK_F1, VK_F2, VK_F3, VK_F4, VK_LEFT, VK_OEM_3, VK_RIGHT, VK_UP, WM_HOTKEY,
};

use crate::game::auto::Auto;

pub fn hotkey() {
    unsafe {
        // Register hotkeys
        RegisterHotKey(null_mut(), 0, 0, VK_F1 as u32);
        RegisterHotKey(null_mut(), 0, 0, VK_F2 as u32);
        RegisterHotKey(null_mut(), 0, 0, VK_F3 as u32);
        RegisterHotKey(null_mut(), 0, 0, VK_F4 as u32);
        RegisterHotKey(null_mut(), 0, 0, VK_END as u32);
        RegisterHotKey(null_mut(), 0, 0, VK_OEM_3 as u32);

        RegisterHotKey(null_mut(), 0, MOD_CONTROL as u32, VK_UP as u32);
        RegisterHotKey(null_mut(), 0, MOD_CONTROL as u32, VK_DOWN as u32);
        RegisterHotKey(null_mut(), 0, MOD_CONTROL as u32, VK_LEFT as u32);
        RegisterHotKey(null_mut(), 0, MOD_CONTROL as u32, VK_RIGHT as u32);

        // Loop to detect hotkeys and release them at the end
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            if msg.message == WM_HOTKEY {
                if (msg.lParam >> 16) as c_int == VK_F1 {
                    println!("全屏开关");
                }
                if (msg.lParam >> 16) as c_int == VK_F2 {
                    println!("VK_F2");
                }
                if (msg.lParam >> 16) as c_int == VK_F3 {
                    println!("VK_F3");
                }
                if (msg.lParam >> 16) as c_int == VK_F4 {
                    println!("VK_F4");
                }
                if (msg.lParam >> 16) as c_int == VK_END {
                    Auto::new().screen_switch();
                }
                if (msg.lParam >> 16) as c_int == VK_OEM_3 {
                    println!("无形秒杀");
                }
                if (msg.lParam >> 16) as c_int == VK_UP {
                    println!("上");
                }
                if (msg.lParam >> 16) as c_int == VK_DOWN {
                    println!("下");
                }
                if (msg.lParam >> 16) as c_int == VK_LEFT {
                    println!("左");
                }
                if (msg.lParam >> 16) as c_int == VK_RIGHT {
                    println!("右");
                }
            } else {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }
    }
}