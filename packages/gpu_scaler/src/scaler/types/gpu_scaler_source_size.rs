use super::GpuScaler;

impl GpuScaler {
    pub fn source_size(&self) -> Option<(u32, u32)> {
        self.source
            .as_ref()
            .map(|source| (source.width, source.height))
    }
}
