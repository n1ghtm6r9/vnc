use crate::test_support::LinearImage;

pub fn transpose(image: &LinearImage) -> LinearImage {
    let width = image.width as usize;
    let mut pixels = Vec::with_capacity(image.pixels.len());
    for x in 0..width {
        for row in image.pixels.chunks_exact(width) {
            pixels.push(row[x]);
        }
    }
    LinearImage {
        width: image.height,
        height: image.width,
        pixels,
    }
}
