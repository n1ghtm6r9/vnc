use super::ColourMatrix;

impl ColourMatrix {
    pub(crate) fn luma_weights(self) -> Option<(f64, f64)> {
        match self {
            ColourMatrix::Identity => None,
            ColourMatrix::Bt601 => Some((0.299, 0.114)),
            ColourMatrix::Bt709 => Some((0.2126, 0.0722)),
            ColourMatrix::Bt2020 => Some((0.2627, 0.0593)),
        }
    }
}
