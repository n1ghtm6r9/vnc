use nvdec::{ChromaLayout, DecodedFrame, PlaneBuffer};

use crate::test_support::PLANES_AS_RGB;

pub fn fake_frame(width: u32, height: u32, value: u8) -> DecodedFrame {
    DecodedFrame {
        width,
        height,
        layout: ChromaLayout::Yuv444,
        colour: PLANES_AS_RGB,
        planes: PlaneBuffer::from(vec![value; (width * height * 3) as usize]),
    }
}
