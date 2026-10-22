use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub(crate) struct AudioReorder {
    pub(crate) ssrc: Option<u32>,
    pub(crate) next: Option<i64>,
    pub(crate) highest: Option<i64>,
    pub(crate) stale: usize,
    pub(crate) waiting: BTreeMap<i64, Vec<u8>>,
}
