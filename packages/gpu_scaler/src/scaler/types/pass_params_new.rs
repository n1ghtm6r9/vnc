use super::PassParams;

impl PassParams {
    pub(crate) fn new(
        origin: [i32; 2],
        input: [u32; 2],
        output: [u32; 2],
        encode_srgb: bool,
    ) -> PassParams {
        let signed = |pair: [u32; 2]| pair.map(|value| i32::try_from(value).unwrap_or(i32::MAX));
        PassParams {
            origin,
            input_size: signed(input),
            output_size: signed(output),
            encode_srgb: u32::from(encode_srgb),
            padding: 0,
        }
    }
}
