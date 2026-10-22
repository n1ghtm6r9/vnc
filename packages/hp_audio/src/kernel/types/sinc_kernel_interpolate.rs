use super::SincKernel;
use crate::{KERNEL_HALF, KERNEL_PHASES, KERNEL_TAPS};

impl SincKernel {
    pub(crate) fn interpolate(&self, frames: &[[f32; 2]], cursor: usize, phase: f64) -> [f32; 2] {
        let position = phase * KERNEL_PHASES as f64;
        let row = (position as usize).min(KERNEL_PHASES - 1);
        let blend = (position - row as f64) as f32;
        let start = cursor + 1 - KERNEL_HALF;
        let mut sum = [0.0_f32; 2];
        for ((frame, lower), upper) in frames[start..start + KERNEL_TAPS]
            .iter()
            .zip(&self.rows[row])
            .zip(&self.rows[row + 1])
        {
            let weight = lower + (upper - lower) * blend;
            sum[0] += frame[0] * weight;
            sum[1] += frame[1] * weight;
        }
        sum
    }
}
