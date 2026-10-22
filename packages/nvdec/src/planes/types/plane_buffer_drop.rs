use std::mem;

use super::PlaneBuffer;

impl Drop for PlaneBuffer {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.take() {
            pool.give_back(mem::take(&mut self.bytes));
        }
    }
}
