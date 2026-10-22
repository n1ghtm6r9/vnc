use rfb_session::{ClientCommand, SessionHandle};

use crate::{FIRST_FRAME_DEADLINE, Snapshot, SnapshotError};

impl Snapshot {
    pub(crate) fn on_quiet(&self, session: &SessionHandle) -> Result<(), SnapshotError> {
        if self.first_frame.is_some() {
            session.send(ClientCommand::RefreshScreen);
            return Ok(());
        }
        if self.started.elapsed() > FIRST_FRAME_DEADLINE {
            return Err(SnapshotError::NoFrame(FIRST_FRAME_DEADLINE));
        }
        Ok(())
    }
}
