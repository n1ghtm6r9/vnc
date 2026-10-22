use wgpu::PresentMode;

use crate::choose_present_mode;

#[test]
fn test_mailbox_is_preferred_when_offered() {
    let mode = choose_present_mode(&[PresentMode::Fifo, PresentMode::Immediate, PresentMode::Mailbox]);
    assert_eq!(mode, PresentMode::Mailbox, "mailbox shows the newest frame without tearing");
}
