use flate2::{Decompress, FlushDecompress, Status};

use crate::DecodeError;

pub(crate) fn inflate_payload(
    stream: &mut Decompress,
    compressed: &[u8],
    inflated: &mut Vec<u8>,
    limit: usize,
) -> Result<usize, DecodeError> {
    if inflated.len() <= limit {
        inflated.resize(limit + 1, 0);
    }
    let input_before = stream.total_in();
    let output_before = stream.total_out();
    let status = stream
        .decompress(compressed, &mut inflated[..=limit], FlushDecompress::Sync)
        .map_err(|error| DecodeError::Corrupt(format!("the zlib stream is invalid: {error}")))?;
    let consumed = usize::try_from(stream.total_in() - input_before).unwrap_or(usize::MAX);
    let produced = usize::try_from(stream.total_out() - output_before).unwrap_or(usize::MAX);
    if status == Status::StreamEnd {
        stream.reset(true);
    }
    if produced > limit {
        return Err(DecodeError::Corrupt(format!(
            "the zlib data inflates beyond the {limit} bytes the rect allows"
        )));
    }
    if consumed < compressed.len() {
        return Err(DecodeError::Corrupt(
            "the zlib payload holds bytes after the end of its stream".to_owned(),
        ));
    }
    Ok(produced)
}
