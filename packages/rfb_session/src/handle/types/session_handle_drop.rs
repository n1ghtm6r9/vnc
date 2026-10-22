use super::SessionHandle;

impl Drop for SessionHandle {
    fn drop(&mut self) {
        self.disconnect();
    }
}
