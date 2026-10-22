#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScaleFilter {
    #[default]
    Smooth,
    Nearest,
}
