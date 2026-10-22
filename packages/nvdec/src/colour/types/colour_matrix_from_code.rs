use super::ColourMatrix;

impl ColourMatrix {
    pub(crate) fn from_code(code: u8) -> Self {
        match code {
            0 => ColourMatrix::Identity,
            4..=6 => ColourMatrix::Bt601,
            9 | 10 => ColourMatrix::Bt2020,
            _ => ColourMatrix::Bt709,
        }
    }
}
