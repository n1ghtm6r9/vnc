pub fn mvs_bits(parts: &[&str]) -> Vec<u8> {
    let bits: Vec<bool> = parts
        .iter()
        .flat_map(|part| part.chars())
        .filter(|symbol| !symbol.is_whitespace())
        .map(|symbol| symbol == '1')
        .collect();
    let mut bytes = vec![0u8; bits.len().div_ceil(8)];
    for (index, set) in bits.into_iter().enumerate() {
        if set {
            bytes[index / 8] |= 0x80 >> (index % 8);
        }
    }
    bytes
}
