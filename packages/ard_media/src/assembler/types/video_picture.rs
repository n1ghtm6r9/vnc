#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoPicture {
    pub don: i64,
    pub stripe: u32,
    pub frame: u16,
    pub key: bool,
    pub data: Vec<u8>,
}
