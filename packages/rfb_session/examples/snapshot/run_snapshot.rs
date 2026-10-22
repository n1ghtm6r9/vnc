use std::{
    sync::mpsc::{self, RecvTimeoutError},
    time::Instant,
};

use rfb_session::start_session;

use crate::{QUIET_WAIT, Snapshot, SnapshotArgs, SnapshotError, connect_options};

pub(crate) fn run_snapshot(args: &SnapshotArgs) -> Result<(), SnapshotError> {
    let mut snapshot = Snapshot::new(Instant::now());
    let (sender, events) = mpsc::channel();
    let session = start_session(
        connect_options(args),
        Box::new(move |event| {
            let _ = sender.send(event);
        }),
    );
    while !snapshot.is_complete() {
        match events.recv_timeout(QUIET_WAIT) {
            Ok(event) => snapshot.handle_event(event)?,
            Err(RecvTimeoutError::Timeout) => snapshot.on_quiet(&session)?,
            Err(RecvTimeoutError::Disconnected) => return Err(SnapshotError::Ended),
        }
    }
    session.disconnect();
    snapshot.save_png(&args.out)?;
    println!("saved {}", args.out.display());
    Ok(())
}
