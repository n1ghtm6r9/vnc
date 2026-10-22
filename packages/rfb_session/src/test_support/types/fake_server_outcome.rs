use super::FakeServer;
use crate::test_support::EVENT_TIMEOUT;

impl<T> FakeServer<T> {
    pub fn outcome(&self) -> T {
        self.outcome
            .recv_timeout(EVENT_TIMEOUT * 2)
            .expect("the fake server finishes its script")
    }
}
