use wgpu::PresentMode;

pub fn choose_present_mode(supported: &[PresentMode]) -> PresentMode {
    if supported.contains(&PresentMode::Mailbox) {
        PresentMode::Mailbox
    } else {
        PresentMode::Fifo
    }
}
