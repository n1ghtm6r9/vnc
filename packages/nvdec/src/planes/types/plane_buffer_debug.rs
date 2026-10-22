use std::fmt;

use super::PlaneBuffer;

impl fmt::Debug for PlaneBuffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PlaneBuffer")
            .field("len", &self.bytes.len())
            .finish_non_exhaustive()
    }
}
