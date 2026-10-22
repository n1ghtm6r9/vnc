use std::mem;

use super::CountingReader;

impl<R> CountingReader<R> {
    pub(crate) fn take_count(&mut self) -> u64 {
        mem::take(&mut self.count)
    }
}
