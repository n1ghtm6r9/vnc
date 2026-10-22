use super::access_units;
use crate::{DecodedFrame, HevcDecoder};

pub fn decode_stream(decoder: &mut HevcDecoder, stream: &[u8]) -> Vec<DecodedFrame> {
    access_units(stream)
        .into_iter()
        .flat_map(|unit| {
            decoder
                .decode(unit)
                .expect("every picture of a clean stream decodes")
        })
        .collect()
}
