use super::PlaneBuffer;

impl From<Vec<u8>> for PlaneBuffer {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes, pool: None }
    }
}
