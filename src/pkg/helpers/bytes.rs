
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