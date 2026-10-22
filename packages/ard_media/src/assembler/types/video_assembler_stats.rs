use super::{AssemblerStats, VideoAssembler};

impl VideoAssembler {
    pub fn stats(&self) -> AssemblerStats {
        self.stats
    }
}
