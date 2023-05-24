pub fn ascii_to_unicode(s: &str) -> Vec<u8> {
    let mut bytes_arr = Vec::new();
    for c in s.chars() {
        let hex_int = c as u16;
        let mut hex_bytes = hex_int.to_le_bytes().to_vec();
        bytes_arr.append(&mut hex_bytes);
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
