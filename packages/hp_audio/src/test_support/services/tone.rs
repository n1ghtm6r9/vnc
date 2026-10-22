use std::f64::consts::TAU;

use crate::SAMPLE_RATE;

pub fn tone(frequency: f64, at: f64) -> f32 {
    (TAU * frequency * at / f64::from(SAMPLE_RATE)).sin() as f32
}
