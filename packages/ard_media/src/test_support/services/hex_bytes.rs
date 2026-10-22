pub fn hex_bytes(text: &str) -> Vec<u8> {
    let digits: Vec<u8> = text.bytes().filter(u8::is_ascii_hexdigit).collect();
    digits
        .chunks(2)
        .map(|pair| {
            let pair = std::str::from_utf8(pair).expect("hex digits are ASCII");
            u8::from_str_radix(pair, 16).expect("two hex digits make a byte")
        })
        .collect()
}
