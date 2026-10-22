use super::VideoAssembler;
use crate::SEQ_MODULUS;

impl VideoAssembler {
    pub(crate) fn start(&mut self, key_don: i64) {
        let shift = if key_don < 0 {
            (-key_don + SEQ_MODULUS - 1) / SEQ_MODULUS * SEQ_MODULUS
        } else {
            0
        };
        let kept = self.pictures.split_off(&key_don);
        self.stats.discarded += self.pictures.len() as u64;
        self.pictures = kept
            .into_iter()
            .map(|(don, picture)| (don + shift, picture))
            .collect();
        self.first = Some(key_don + shift);
        self.next = Some(key_don + shift);
    }
}
