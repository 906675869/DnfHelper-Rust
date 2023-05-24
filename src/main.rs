pub mod game;
pub mod pkg;

use pkg::helpers::array;
use pkg::helpers::string;
use pkg::helpers::bytes;
use game::address;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let check_data = 3;
    if array::in_array(check_data, &data) {
        println!("{} is in the array.", check_data);
    } else {
        println!("{} is not in the array.", check_data);
    }

    println!("{} is",address::RW_CALL);


    let s = "格兰迪发电站";
    let bytes_arr = string::ascii_to_unicode(s);
    println!("{:?}", bytes_arr);

    let str = string::unicode_to_ascii(&bytes_arr);

    println!("{}",str);


    let old_byte_arr: Vec<u8> = vec![0x01, 0x02];
    let new_byte_arr: &[&[u8]] = &[&[0x03, 0x04], &[0x05, 0x06],&[0x07, 0x08]];

    let result = bytes::add_byte_arr(old_byte_arr, new_byte_arr);

    println!("{:?}", result);
}