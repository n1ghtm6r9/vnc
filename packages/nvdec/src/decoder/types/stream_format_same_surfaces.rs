use super::StreamFormat;

impl StreamFormat {
    pub(crate) fn same_surfaces(&self, other: &StreamFormat) -> bool {
        (
            self.coded_width,
            self.coded_height,
            self.chroma,
            self.decode_surfaces,
        ) == (
            other.coded_width,
            other.coded_height,
            other.chroma,
            other.decode_surfaces,
        )
    }
}
