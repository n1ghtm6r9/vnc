use std::time::Instant;

use winit::event_loop::{ActiveEventLoop, ControlFlow};

use super::{ClientApp, LinkState};

impl ClientApp {
    pub(super) fn schedule_wakeup(&mut self, event_loop: &ActiveEventLoop) {
        self.reconnect_when_due();
        self.resize_display_when_due();
        if self.repaint_at.is_some_and(|at| at <= Instant::now()) {
            self.repaint_at = None;
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        let reconnect_at = match self.link {
            LinkState::Waiting { at, .. } => Some(at),
            _ => None,
        };
        let wake_at = [self.repaint_at, reconnect_at, self.display_resize_at].into_iter().flatten().min();
        event_loop.set_control_flow(wake_at.map_or(ControlFlow::Wait, ControlFlow::WaitUntil));
    }
}
