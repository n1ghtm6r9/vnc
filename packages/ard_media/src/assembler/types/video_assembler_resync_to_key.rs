use super::{AssemblerEvent, VideoAssembler};

impl VideoAssembler {
    pub(crate) fn resync_to_key(&mut self, events: &mut Vec<AssemblerEvent>) -> bool {
        let Some(next) = self.next else {
            return false;
        };
        let Some(key_don) = self
            .pictures
            .iter()
            .find(|(don, picture)| **don > next && picture.key && picture.data.is_some())
            .map(|(don, _)| *don)
        else {
            return false;
        };
        let kept = self.pictures.split_off(&key_don);
        self.stats.skipped += self.pictures.len() as u64;
        self.pictures = kept;
        self.next = Some(key_don);
        self.emit(events);
        true
    }
}
