use super::PlanePool;

pub struct PlaneBuffer {
    pub(crate) bytes: Vec<u8>,
    pub(crate) pool: Option<PlanePool>,
}
