use crate::PIXELS_PER_WHEEL_STEP;

pub(crate) fn take_whole_steps(carry: &mut f64, delta: f64) -> isize {
    if !delta.is_finite() {
        return 0;
    }
    let total = *carry + delta;
    let remainder = total % PIXELS_PER_WHEEL_STEP;
    *carry = remainder;
    ((total - remainder) / PIXELS_PER_WHEEL_STEP) as isize
}
