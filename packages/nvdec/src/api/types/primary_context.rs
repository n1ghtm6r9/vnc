use crate::{CuContext, CuDevice};

#[derive(Clone, Copy)]
pub(crate) struct PrimaryContext {
    pub(crate) device: CuDevice,
    pub(crate) context: CuContext,
}
