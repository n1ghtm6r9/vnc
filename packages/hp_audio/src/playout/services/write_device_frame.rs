pub(crate) fn write_device_frame(device_frame: &mut [f32], [left, right]: [f32; 2]) {
    match device_frame {
        [] => {}
        [mono] => *mono = (left + right) / 2.0,
        [front_left, front_right, rest @ ..] => {
            *front_left = left;
            *front_right = right;
            rest.fill(0.0);
        }
    }
}
