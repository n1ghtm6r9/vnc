use crate::KERNEL_TAPS;

pub(crate) struct SincKernel {
    pub(crate) rows: Vec<[f32; KERNEL_TAPS]>,
}
