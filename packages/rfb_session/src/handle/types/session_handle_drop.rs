use super::SessionHandle;
use crate::WRITER_FINISH_TIMEOUT;

impl Drop for SessionHandle {
    fn drop(&mut self) {
        self.disconnect();
        let _ = self.writer_finished.recv_timeout(WRITER_FINISH_TIMEOUT);
    }
}
