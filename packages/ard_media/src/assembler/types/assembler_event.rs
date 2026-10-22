use super::VideoPicture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssemblerEvent {
    Picture(VideoPicture),
    Stalled {
        expected_don: Option<i64>,
        buffered: usize,
    },
}
