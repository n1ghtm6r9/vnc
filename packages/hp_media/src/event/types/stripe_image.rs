use nvdec::DecodedFrame;

#[derive(Debug, Clone, PartialEq)]
pub struct StripeImage {
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub frame: DecodedFrame,
}
