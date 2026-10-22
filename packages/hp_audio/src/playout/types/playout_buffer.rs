use std::collections::VecDeque;

use super::PlayoutStats;
use crate::SincKernel;

pub(crate) struct PlayoutBuffer {
    pub(crate) frames: VecDeque<[f32; 2]>,
    pub(crate) cursor: usize,
    pub(crate) phase: f64,
    pub(crate) playing: bool,
    pub(crate) smoothed_level: f64,
    pub(crate) fade_in: usize,
    pub(crate) output_rate: u32,
    pub(crate) kernel: SincKernel,
    pub(crate) stats: PlayoutStats,
}
