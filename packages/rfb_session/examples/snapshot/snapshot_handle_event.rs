use rfb_session::SessionEvent;

use crate::{Snapshot, SnapshotError, encoding_name};

impl Snapshot {
    pub(crate) fn handle_event(&mut self, event: SessionEvent) -> Result<(), SnapshotError> {
        match event {
            SessionEvent::Connected {
                width,
                height,
                name,
                apple_remote_desktop,
            } => {
                println!(
                    "connected to {name:?}, {width}x{height}, Apple Remote Desktop mode {apple_remote_desktop}, after {:?}",
                    self.started.elapsed()
                );
                self.resize(width, height);
            }
            SessionEvent::Resized { width, height } => {
                println!("the desktop is now {width}x{height}");
                self.resize(width, height);
            }
            SessionEvent::FrameUpdated { patches } => {
                patches.iter().for_each(|patch| self.draw_patch(patch));
                if self.first_frame.is_none() {
                    let elapsed = self.started.elapsed();
                    println!("first frame after {elapsed:?}");
                    self.first_frame = Some(elapsed);
                }
            }
            SessionEvent::Stats(stats) => {
                println!(
                    "{} bytes/s, {:.1} updates/s, last encoding: {}",
                    stats.bytes_per_second,
                    stats.updates_per_second,
                    encoding_name(stats.last_encoding)
                );
                self.stats = Some(stats);
            }
            SessionEvent::HighPerformance(state) => println!("High Performance: {state:?}"),
            SessionEvent::Disconnected { error } => {
                return Err(error.map_or(SnapshotError::Ended, SnapshotError::Session));
            }
            SessionEvent::CursorChanged(_)
            | SessionEvent::SystemCursorChanged(_)
            | SessionEvent::CursorVisibilityChanged { .. }
            | SessionEvent::Bell
            | SessionEvent::ClipboardText(_)
            | SessionEvent::InputSource(_)
            | SessionEvent::DisplayGeometry(_)
            | SessionEvent::PictureStripe(_) => {}
        }
        Ok(())
    }
}
