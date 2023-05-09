fn ascii_to_unicode(s: &str) -> Vec<u8> {
    let mut new_bytes = Vec::new();
    for c in s.chars() {
        let code_point = c as u32;
        new_bytes.extend_from_slice(&(code_point.to_be_bytes()));
    }
    new_bytes
}

fn unicode_to_string(b: &[u8]) -> String {
    let mut rune_array = Vec::new();
    for i in (0..b.len()).step_by(2) {
        if b[i] == 0 && b[i + 1] == 0 {
            break;
        }
        let code_point = ((b[i] as u32) << 8) | (b[i+1] as u32);
        rune_array.push(std::char::from_u32(code_point).unwrap());
    }
    rune_array.iter().collect()
}
