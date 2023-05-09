use std::mem::size_of;
use std::slice;

fn int_to_byte_arr<T: Into<u64> + From<u64>>(num: T) -> Vec<u8> {
    let size = size_of::<T>();
    let mut byte_arr = vec![0; size];
    if size == 4 {
        let num_u32 = num.into() as u32;
        byte_arr.copy_from_slice(&num_u32.to_le_bytes());
    }
    if size == 8 {
        let num_u64 = num.into() as u64;
        byte_arr.copy_from_slice(&num_u64.to_le_bytes());
    }
    byte_arr
}

fn byte_arr_to_int<T: Into<u64> + From<u64>>(byte_arr: &[u8]) -> T {
    let size = size_of::<T>();
    let data = match size {
        4 => T::from(u32::from_le_bytes(byte_arr.try_into().unwrap())),
        8 => T::from(u64::from_le_bytes(byte_arr.try_into().unwrap())),
        _ => panic!("Unsupported integer size: {}", size),
    };
    data
}

fn byte_arr_to_float<T: From<f32> + From<f64>>(byte_arr: &[u8]) -> T {
    let size = size_of::<T>();
    let data = match size {
        4 => f32::from_bits(u32::from_le_bytes(byte_arr.try_into().unwrap())),
        8 => f64::from_bits(u64::from_le_bytes(byte_arr.try_into().unwrap())),
        _ => panic!("Unsupported float size: {}", size),
    };
    T::from(data)
}

fn float_to_byte_arr<T: Into<f32> + Into<f64>>(float: T) -> Vec<u8> {
    let size = size_of::<T>();
    let mut byte_arr = vec![0; size];
    if size == 4 {
        let bits = float.into().to_bits();
        byte_arr.copy_from_slice(&bits.to_le_bytes());
    }
    if size == 8 {
        let bits = float.into().to_bits();
        byte_arr.copy_from_slice(&bits.to_le_bytes());
    }
    byte_arr
}


fn add_byte_arr(old_byte_arr: Vec<u8>, new_byte_arr: &[&[u8]]) -> Vec<u8> {
    let mut res = old_byte_arr;
    for arr in new_byte_arr {
        res.extend_from_slice(arr);
    }
    res
}