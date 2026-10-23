use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use crate::{Outgoing, SessionControl, SessionHandle};

#[test]
fn test_dropping_the_handle_waits_for_the_writer_to_finish() {
    let (outgoing, queue) = mpsc::channel();
    let (writer_finished, finished) = mpsc::channel();
    let flushed = Arc::new(AtomicBool::new(false));
    let writer_flushed = Arc::clone(&flushed);
    let writer = thread::spawn(move || {
        while queue.recv().is_ok_and(|message| message != Outgoing::Stop) {}
        thread::sleep(Duration::from_millis(20));
        writer_flushed.store(true, Ordering::SeqCst);
        let _ = writer_finished.send(());
    });
    let session = SessionHandle::new(outgoing, Arc::new(SessionControl::default()), finished);

    drop(session);
    let flushed_before_the_drop_returned = flushed.load(Ordering::SeqCst);
    writer.join().expect("the writer thread ends");

    assert!(
        flushed_before_the_drop_returned,
        "dropping the handle returns only after the writer has sent its last messages"
    );
}
