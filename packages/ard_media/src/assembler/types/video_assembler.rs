use std::{collections::BTreeMap, time::Duration};

use super::{AssemblerStats, PendingPicture};

pub struct VideoAssembler {
    pub(crate) base_ssrc: u32,
    pub(crate) stripes: u32,
    pub(crate) stall_after: Duration,
    pub(crate) pictures: BTreeMap<i64, PendingPicture>,
    pub(crate) reference: Option<i64>,
    pub(crate) first: Option<i64>,
    pub(crate) next: Option<i64>,
    pub(crate) stalled: bool,
    pub(crate) stats: AssemblerStats,
}
