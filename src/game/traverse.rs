use std::thread;
use crate::GLOBAL;
use crate::helpers::common::sleep;

pub fn thread_switch() {
    let mut cfg = GLOBAL.lock().unwrap();
    cfg.auto_switch = !cfg.auto_switch;
    if cfg.auto_switch {
        thread::spawn(|| {
            loop {
                println!("Thread {}", 1);
                sleep(500);
            }
        });
        println!("Global variable value is {}", cfg.auto_switch);
    } else {
        cfg.auto_switch = false;
        println!("Global variable value is {}", cfg.auto_switch);
    }
}