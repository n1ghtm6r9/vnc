use gpu_scaler::{YuvColour, YuvLayout, YuvStripe};
use rfb_session::{ChromaLayout, StripeImage};

pub fn yuv_stripe(stripe: &StripeImage) -> YuvStripe<'_> {
    let frame = &stripe.frame;
    YuvStripe {
        y: stripe.y,
        width: stripe.width,
        height: stripe.height,
        plane_width: frame.width,
        plane_height: frame.height,
        layout: match frame.layout {
            ChromaLayout::Yuv444 => YuvLayout::Planar444,
            ChromaLayout::Nv12 => YuvLayout::Nv12,
        },
        colour: YuvColour { rows: frame.colour.rows, primaries: frame.colour.primaries },
        planes: &frame.planes,
    }
}
