pub fn pixel_at(image: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let start = (y as usize * width as usize + x as usize) * 4;
    [
        image[start],
        image[start + 1],
        image[start + 2],
        image[start + 3],
    ]
}
