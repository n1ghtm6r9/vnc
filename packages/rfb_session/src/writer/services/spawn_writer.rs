use std::{io, sync::mpsc::Sender, thread};

use crate::SessionWriter;

pub(crate) fn spawn_writer(writer: SessionWriter, finished: Sender<()>) -> io::Result<()> {
    thread::Builder::new()
        .name("rfb-writer".to_owned())
        .spawn(move || {
            writer.run();
            let _ = finished.send(());
        })?;
    Ok(())
}
