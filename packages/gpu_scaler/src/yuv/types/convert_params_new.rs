use super::{ConvertParams, YuvLayout, YuvStripe};

impl ConvertParams {
    pub(crate) fn new(stripe: &YuvStripe<'_>) -> ConvertParams {
        ConvertParams {
            origin: [0, i32::try_from(stripe.y).unwrap_or(i32::MAX)],
            chroma_shift: stripe.layout.chroma_shift(),
            interleaved_chroma: u32::from(stripe.layout == YuvLayout::Nv12),
            rows: stripe.colour.rows,
            primaries: stripe
                .colour
                .primaries
                .map(|[red, green, blue]| [red, green, blue, 0.0]),
        }
    }
}
