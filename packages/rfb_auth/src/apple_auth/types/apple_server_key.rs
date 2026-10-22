pub(crate) struct AppleServerKey {
    pub(crate) generator: [u8; 2],
    pub(crate) prime: Vec<u8>,
    pub(crate) public: Vec<u8>,
}
