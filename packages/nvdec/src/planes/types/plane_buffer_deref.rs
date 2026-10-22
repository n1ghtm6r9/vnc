use std::ops::Deref;

use super::PlaneBuffer;

impl Deref for PlaneBuffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}
