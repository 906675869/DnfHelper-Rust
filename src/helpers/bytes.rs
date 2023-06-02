use std::convert::TryInto;
use std::mem;

// 合并字节集
// let old_byte_arr: Vec<u8> = vec![0x01, 0x02];
// let new_byte_arr: &[&[u8]] = &[&[0x03, 0x04], &[0x05, 0x06],&[0x07, 0x08]];
pub fn add_byte_arr(old_byte_arr: Vec<u8>, new_byte_arr: &[&[u8]]) -> Vec<u8> {
    let mut res = old_byte_arr;
    for arr in new_byte_arr {
        res.extend_from_slice(arr);
    }
    res
}


// int_to_byte_arr 整数转byte数组
pub fn int_to_byte_arr<T: Into<u64>>(num: T) -> Vec<u8> {
    let mut byte_arr = vec![0; mem::size_of::<T>()];
    match std::mem::size_of::<T>() {
        2 => byte_arr.copy_from_slice(&u16::to_le_bytes(num.into() as u16)),
        4 => byte_arr.copy_from_slice(&u32::to_le_bytes(num.into() as u32)),
        8 => byte_arr.copy_from_slice(&u64::to_le_bytes(num.into())),
        _ => panic!("Unsupported integer size"),
    }
    byte_arr
}


pub fn byte_arr_to_float<T: From<f32> + From<f64> + Default>(byte_arr: &[u8]) -> T {
    match std::mem::size_of::<T>() {
        4 => {
            let data = u32::from_le_bytes(byte_arr.try_into().unwrap());
            T::from(f32::from_bits(data))
        }
        8 => {
            let data = u64::from_le_bytes(byte_arr.try_into().unwrap());
            T::from(f64::from_bits(data))
        }
        _ => panic!("Unsupported type"),
    }
}
