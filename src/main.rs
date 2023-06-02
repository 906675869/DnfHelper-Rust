use winapi::shared::minwindef::LPVOID;

use dnf_helper_rust::driver::memory_rw::MemoryRw;
use dnf_helper_rust::game::{address, init};
use dnf_helper_rust::helpers::{common, process};

fn main() {


    // let process_id = process::get_process_id("123.exe");
    // if process_id == 0 {
    //     common::show_message_box("请打开dnf后运行", "DnfHelper");
    //     std::process::exit(0);
    // }
    //
    // let mut rw = MemoryRw::new();
    // rw.set_global_id(process_id);
    //
    // // 技能免消耗
    // rw.write_process_memory(address::JN_SW_ADDR as LPVOID, &vec![144, 144, 144, 144, 144]);

    println!("加载成功-欢迎使用");
    println!("当前时间：{}", common::get_now_date());

    init::hotkey();
}
