use std::sync::Mutex;

use lazy_static::lazy_static;

pub mod game;
pub mod helpers;
pub mod driver;

// 定义一个结构体作为全局变量
pub struct GlobalData {
    pub auto_switch: bool,
}

// 使用 Mutex 包裹结构体，以便多线程安全
lazy_static! {
    pub static ref GLOBAL: Mutex<GlobalData> = Mutex::new(GlobalData {
        auto_switch: false
    });
}