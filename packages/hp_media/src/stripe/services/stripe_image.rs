use nvdec::DecodedFrame;

use crate::StripeImage;

pub(crate) fn stripe_image(
    frame: DecodedFrame,
    stripe: u32,
    picture_width: u32,
    picture_height: u32,
) -> Option<StripeImage> {
    let y = stripe.checked_mul(frame.height)?;
    let height = frame.height.min(picture_height.checked_sub(y)?);
    let width = frame.width.min(picture_width);
    if width == 0 || height == 0 {
        return None;
    }
    Some(StripeImage {
        y,
        width,
        height,
        frame,
    })
}
