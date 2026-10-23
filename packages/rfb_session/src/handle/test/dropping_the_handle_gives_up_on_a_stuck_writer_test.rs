use std::{
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};

use crate::{SessionControl, SessionHandle};

#[test]
fn test_dropping_the_handle_gives_up_on_a_stuck_writer() {
    let (outgoing, _queue) = mpsc::channel();
    let (_writer_finished, finished) = mpsc::channel();
    let session = SessionHandle::new(outgoing, Arc::new(SessionControl::default()), finished);
    let started = Instant::now();

    drop(session);

    assert!(
        started.elapsed() < Duration::from_secs(2),
        "dropping the handle stops waiting for a writer that never finishes"
    );
}
