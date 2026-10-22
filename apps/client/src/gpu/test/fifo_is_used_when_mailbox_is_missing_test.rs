use wgpu::PresentMode;

use crate::choose_present_mode;

#[test]
fn test_fifo_is_used_when_mailbox_is_missing() {
    let mode = choose_present_mode(&[PresentMode::Immediate, PresentMode::Fifo]);
    assert_eq!(mode, PresentMode::Fifo, "fifo is always supported and never tears");
}
