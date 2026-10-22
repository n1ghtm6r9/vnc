use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::centred_position;

#[test]
fn test_the_window_is_centred_on_its_monitor() {
    let second_monitor = PhysicalPosition::new(2560, 0);
    let monitor_size = PhysicalSize::new(2560, 1440);

    let centred = centred_position(second_monitor, monitor_size, PhysicalSize::new(2228, 1296));
    let oversized = centred_position(second_monitor, monitor_size, PhysicalSize::new(3000, 1600));

    assert_eq!(
        centred,
        PhysicalPosition::new(2560 + 166, 72),
        "the window sits in the middle of the monitor it is on"
    );
    assert_eq!(
        oversized,
        second_monitor,
        "a window larger than its monitor starts at the corner of the monitor, so its title bar stays reachable"
    );
}
