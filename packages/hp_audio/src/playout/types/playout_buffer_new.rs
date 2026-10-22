use std::collections::VecDeque;

use super::{PlayoutBuffer, PlayoutStats};
use crate::{KERNEL_HALF, PLAYOUT_TARGET, SincKernel, output_cutoff};

impl PlayoutBuffer {
    pub(crate) fn new(output_rate: u32) -> PlayoutBuffer {
        PlayoutBuffer {
            frames: VecDeque::from(vec![[0.0; 2]; KERNEL_HALF - 1]),
            cursor: KERNEL_HALF - 1,
            phase: 0.0,
            playing: false,
            smoothed_level: PLAYOUT_TARGET as f64,
            fade_in: 0,
            output_rate,
            kernel: SincKernel::new(output_cutoff(output_rate)),
            stats: PlayoutStats::default(),
        }
    }
}
