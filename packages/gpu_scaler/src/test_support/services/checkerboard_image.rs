pub fn checkerboard_image(width: u32, height: u32) -> Vec<u8> {
    let mut image = Vec::with_capacity(width as usize * height as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let level = if (x + y) % 2 == 0 { 255 } else { 0 };
            image.extend_from_slice(&[level, level, level, 255]);
        }
    }
    image
}
