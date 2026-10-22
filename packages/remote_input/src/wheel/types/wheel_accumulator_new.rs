use super::WheelAccumulator;

impl WheelAccumulator {
    pub const fn new() -> WheelAccumulator {
        WheelAccumulator {
            vertical: 0.0,
            horizontal: 0.0,
        }
    }
}
