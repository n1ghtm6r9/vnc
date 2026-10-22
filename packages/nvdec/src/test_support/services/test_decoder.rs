use crate::{HevcDecoder, NvdecError};

pub fn test_decoder() -> Option<HevcDecoder> {
    match HevcDecoder::new() {
        Ok(decoder) => Some(decoder),
        Err(
            error @ (NvdecError::Load(_)
            | NvdecError::Cuda {
                call: "cuInit" | "cuDeviceGet",
                ..
            }),
        ) => {
            println!("no NVIDIA video decoder here: {error}");
            None
        }
        Err(error) => panic!("the NVIDIA video decoder is installed but did not start: {error}"),
    }
}
