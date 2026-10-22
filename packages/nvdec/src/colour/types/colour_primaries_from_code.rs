use super::ColourPrimaries;

impl ColourPrimaries {
    pub(crate) fn from_code(code: u8) -> Self {
        match code {
            12 => ColourPrimaries::DisplayP3,
            _ => ColourPrimaries::Bt709,
        }
    }
}
