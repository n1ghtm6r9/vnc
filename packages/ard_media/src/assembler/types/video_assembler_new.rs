use std::collections::BTreeMap;

use super::{AssemblerStats, VideoAssembler};
use crate::PICTURE_STALL_AFTER;

impl VideoAssembler {
    pub fn new(base_ssrc: u32, stripes: u32) -> Self {
        Self {
            base_ssrc,
            stripes,
            stall_after: PICTURE_STALL_AFTER,
            pictures: BTreeMap::new(),
            reference: None,
            first: None,
            next: None,
            stalled: false,
            stats: AssemblerStats::default(),
        }
    }
}
