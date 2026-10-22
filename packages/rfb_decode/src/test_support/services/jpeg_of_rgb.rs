use jpeg_encoder::{ColorType, Encoder};

pub fn jpeg_of_rgb(width: u16, height: u16, rgb: &[u8]) -> Vec<u8> {
    let mut jpeg = Vec::new();
    Encoder::new(&mut jpeg, 100)
        .encode(rgb, width, height, ColorType::Rgb)
        .expect("the test image encodes");
    jpeg
}
