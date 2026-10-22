pub fn seeded_rgb_bytes(seed: u8, length: u8) -> Vec<u8> {
    (0..length)
        .map(|index| index.wrapping_mul(5).wrapping_add(seed))
        .collect()
}
