#[derive(Debug, Clone, Default, PartialEq)]
pub struct WheelAccumulator {
    pub(super) vertical: f64,
    pub(super) horizontal: f64,
}
