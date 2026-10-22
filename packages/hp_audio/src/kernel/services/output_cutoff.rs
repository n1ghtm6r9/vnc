use crate::{KERNEL_CUTOFF, SAMPLE_RATE};

pub(crate) fn output_cutoff(output_rate: u32) -> f64 {
    KERNEL_CUTOFF * (f64::from(output_rate) / f64::from(SAMPLE_RATE)).min(1.0)
}
