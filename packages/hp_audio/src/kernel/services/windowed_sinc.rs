use std::f64::consts::TAU;

use super::bessel_i0;
use crate::{KAISER_BETA, KERNEL_HALF};

pub(crate) fn windowed_sinc(offset: f64, cutoff: f64) -> f64 {
    let edge = offset / KERNEL_HALF as f64;
    if edge.abs() >= 1.0 {
        return 0.0;
    }
    let window = bessel_i0(KAISER_BETA * (1.0 - edge * edge).sqrt()) / bessel_i0(KAISER_BETA);
    let angle = TAU * cutoff * offset;
    let sinc = if angle.abs() < f64::EPSILON {
        1.0
    } else {
        angle.sin() / angle
    };
    2.0 * cutoff * sinc * window
}
