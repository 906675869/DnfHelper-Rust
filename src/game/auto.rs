use std::thread;

use crate::helpers::common;

pub struct Auto {
    auto_switch: bool,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Auto {
    pub fn new() -> Self {
        Self {
            auto_switch: false,
            thread_handle: None,
        }
    }

    pub fn thread_switch(&mut self) {
        self.auto_switch = !self.auto_switch;
        if self.auto_switch {
            let handle = thread::spawn(move || {
                loop {
                    // 检查是否需要强制终止线程
                    if thread::panicking() {
                        // 线程被强制终止
                        return;
                    }
                    println!("时间: {}", common::get_now_date());
                    common::sleep(500);
                }
            });
            // 保存线程句柄
            self.thread_handle = Some(handle);
            println!("Global variable value is {}", self.auto_switch);
        } else {
            // 如果标志为 false，则停止线程
            if let Some(handle) = self.thread_handle.take() {
                handle.thread().unpark();
                self.auto_switch = false;
            }

            println!("Global variable value is {}", self.auto_switch);
        }
    }
}

