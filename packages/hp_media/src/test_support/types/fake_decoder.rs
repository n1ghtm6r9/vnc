#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FakeDecoder {
    pub broken: bool,
    pub silent: bool,
}
