pub fn ascii_to_unicode(s: &str) -> Vec<u8> {
    let mut bytes_arr = Vec::new();
    for c in s.chars() {
        let hex_int = c as u32;
        let mut hex_bytes = hex_int.to_le_bytes().to_vec(); // 将整数转换为小端字节序的字节数组，并转换为 Vec<u8>
        bytes_arr.append(&mut hex_bytes); // 将字节数组追加到结果数组中
    }
    bytes_arr
}

pub fn unicode_to_ascii(ls: &[u8]) -> String {
    let mut text = String::new();
    for i in (0..ls.len()).step_by(2) {
        if ls[i] == 0 && ls[i + 1] == 0 {
            break;
        }
        let a = (ls[i+1] as u32) << 8;
        let b = ls[i] as u32;
        text.push(std::char::from_u32(a | b).unwrap());
    }
    text
}
