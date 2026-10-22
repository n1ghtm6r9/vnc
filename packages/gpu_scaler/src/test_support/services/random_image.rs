pub fn random_image(width: u32, height: u32, seed: u32) -> Vec<u8> {
    let mut state = seed.max(1);
    let mut next_byte = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        (state >> 24) as u8
    };
    let mut image = Vec::with_capacity(width as usize * height as usize * 4);
    for _ in 0..width as usize * height as usize {
        image.extend_from_slice(&[next_byte(), next_byte(), next_byte(), 255]);
    }
    image
}
