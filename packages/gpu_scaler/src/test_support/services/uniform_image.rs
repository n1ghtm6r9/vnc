pub fn uniform_image(width: u32, height: u32, colour: [u8; 4]) -> Vec<u8> {
    colour.repeat(width as usize * height as usize)
}
