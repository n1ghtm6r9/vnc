use std::ops::DerefMut;

use super::PlaneBuffer;

impl DerefMut for PlaneBuffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
