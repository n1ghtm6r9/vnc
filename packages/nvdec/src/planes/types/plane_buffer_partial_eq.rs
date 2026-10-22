use super::PlaneBuffer;

impl PartialEq for PlaneBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}
