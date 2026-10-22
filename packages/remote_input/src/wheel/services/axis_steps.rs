use std::iter;

use crate::WheelStep;

pub(crate) fn axis_steps(
    count: isize,
    positive: WheelStep,
    negative: WheelStep,
) -> impl Iterator<Item = WheelStep> {
    let direction = if count > 0 { positive } else { negative };
    iter::repeat_n(direction, count.unsigned_abs())
}
