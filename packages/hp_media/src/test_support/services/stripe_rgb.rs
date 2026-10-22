use nvdec::ChromaLayout;

use crate::StripeImage;

pub fn stripe_rgb(image: &StripeImage, x: u32, y: u32) -> [u8; 3] {
    let frame = &image.frame;
    let width = frame.width as usize;
    let luma_len = width * frame.height as usize;
    let (x, y) = (x as usize, y as usize);
    let luma = frame.planes[y * width + x];
    let (blue, red) = match frame.layout {
        ChromaLayout::Yuv444 => (
            frame.planes[luma_len + y * width + x],
            frame.planes[2 * luma_len + y * width + x],
        ),
        ChromaLayout::Nv12 => {
            let pair = luma_len + (y / 2) * width.div_ceil(2) * 2 + (x / 2) * 2;
            (frame.planes[pair], frame.planes[pair + 1])
        }
    };
    frame.colour.encoded_rgb(luma, blue, red)
}
