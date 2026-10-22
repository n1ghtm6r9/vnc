use std::{collections::HashMap, time::Instant};

use super::PayloadPart;

pub(crate) struct PendingPicture {
    pub(crate) stripe: u32,
    pub(crate) frame: u16,
    pub(crate) count: u16,
    pub(crate) parts: HashMap<u16, PayloadPart>,
    pub(crate) data: Option<Vec<u8>>,
    pub(crate) key: bool,
    pub(crate) first_at: Instant,
    pub(crate) done_at: Option<Instant>,
}
