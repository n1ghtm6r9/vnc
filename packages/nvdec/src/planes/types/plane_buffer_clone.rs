use super::PlaneBuffer;

impl Clone for PlaneBuffer {
    fn clone(&self) -> Self {
        PlaneBuffer::from(self.bytes.clone())
    }
}
