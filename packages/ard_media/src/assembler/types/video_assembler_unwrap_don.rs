use super::VideoAssembler;
use crate::extend_seq;

impl VideoAssembler {
    pub(crate) fn unwrap_don(&mut self, don: u16) -> i64 {
        let reference = *self.reference.get_or_insert(i64::from(don));
        extend_seq(self.next.unwrap_or(reference), don)
    }
}
