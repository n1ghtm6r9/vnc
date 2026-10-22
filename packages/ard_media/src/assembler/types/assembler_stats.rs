#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AssemblerStats {
    pub packets: u64,
    pub ignored: u64,
    pub malformed: u64,
    pub duplicates: u64,
    pub pictures: u64,
    pub discarded: u64,
    pub skipped: u64,
}
