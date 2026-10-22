use std::array;

use super::SincKernel;
use crate::{KERNEL_HALF, KERNEL_PHASES, KERNEL_TAPS, windowed_sinc};

impl SincKernel {
    pub(crate) fn new(cutoff: f64) -> SincKernel {
        let rows = (0..=KERNEL_PHASES)
            .map(|row| {
                let shift = row as f64 / KERNEL_PHASES as f64;
                let weights: [f64; KERNEL_TAPS] = array::from_fn(|tap| {
                    windowed_sinc(tap as f64 + 1.0 - KERNEL_HALF as f64 - shift, cutoff)
                });
                let total: f64 = weights.iter().sum();
                weights.map(|weight| (weight / total) as f32)
            })
            .collect();
        SincKernel { rows }
    }
}
