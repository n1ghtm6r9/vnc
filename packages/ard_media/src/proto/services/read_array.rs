use super::read_slice;
use crate::MediaError;

pub(crate) fn read_array<const N: usize>(
    bytes: &[u8],
    position: &mut usize,
) -> Result<[u8; N], MediaError> {
    let slice = read_slice(bytes, position, N as u64)?;
    Ok(slice.try_into().expect("the slice has the array's length"))
}
