use crate::MediaError;

pub(crate) fn random_array<const N: usize>() -> Result<[u8; N], MediaError> {
    let mut bytes = [0u8; N];
    getrandom::fill(&mut bytes).map_err(|error| MediaError::Random(error.to_string()))?;
    Ok(bytes)
}
