use super::YuvLayout;

impl YuvLayout {
    pub(crate) fn chroma_shift(self) -> u32 {
        match self {
            YuvLayout::Planar444 => 0,
            YuvLayout::Nv12 => 1,
        }
    }
}
