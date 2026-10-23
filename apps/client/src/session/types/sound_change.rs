#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundChange {
    Play(bool),
    LeaveHighPerformance,
    EnterHighPerformance,
    Keep,
}
