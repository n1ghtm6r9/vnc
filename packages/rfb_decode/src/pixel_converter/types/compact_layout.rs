#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompactLayout {
    Full,
    LeastSignificantThree,
    MostSignificantThree,
}
