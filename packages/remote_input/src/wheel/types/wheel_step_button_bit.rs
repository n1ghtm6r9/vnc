use super::WheelStep;

impl WheelStep {
    pub fn button_bit(self) -> u8 {
        match self {
            WheelStep::Up => 8,
            WheelStep::Down => 16,
            WheelStep::Left => 32,
            WheelStep::Right => 64,
        }
    }
}
