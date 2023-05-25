use game::address;
use pkg::driver::memory_rw::{read_process_memory, virtual_alloc_ex, write_process_memory};
use pkg::helpers::{array, bytes, string};

pub mod game;
pub mod pkg;

fn main() {
    let data = [1, 2, 3, 4, 5];
    let check_data = 3;
    if array::in_array(check_data, &data) {
        println!("{} is in the array.", check_data);
    } else {
        println!("{} is not in the array.", check_data);
    }

    println!("人物基质: {}", address::RW_CALL);


    let s = "格兰迪发电站";
    let bytes_arr = string::ascii_to_unicode(s);
    println!("{:?}", bytes_arr);

    let str = string::unicode_to_ascii(&bytes_arr);

    println!("{}", str);


    let old_byte_arr: Vec<u8> = vec![0x01, 0x02];
    let new_byte_arr: &[&[u8]] = &[&[0x03, 0x04], &[0x05, 0x06], &[0x07, 0x08]];

    let result = bytes::add_byte_arr(old_byte_arr, new_byte_arr);

    println!("{:?}", result);


    mem();
}


fn mem() {
    // 示例：从进程 ID 为 1234 的进程内存地址 0x00400000 处读取 4 个字节
    let process_id = 16192;

    let address = virtual_alloc_ex(process_id);
    println!("{:?}", address);

    let buffer = [1u8, 2u8, 3u8, 4u8];
    if write_process_memory(process_id, address, &buffer) {
        println!("写入成功");
        println!("{:?}", buffer);
    } else {
        println!("写入失败");
    }


    let mut buffer: [u8; 4] = [0u8; 4];
    if read_process_memory(process_id, address, &mut buffer) {
        println!("读取成功");
        println!("{:?}", buffer);

        let num32: u32 = u32::from_ne_bytes(buffer);
        let fl32: f32 = f32::from_ne_bytes(buffer);

        println!("{}", num32);
        println!("{}", fl32);
    } else {
        println!("读取失败");
    }
}