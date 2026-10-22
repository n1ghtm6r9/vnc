use std::{io, thread};

use crate::SessionWriter;

pub(crate) fn spawn_writer(writer: SessionWriter) -> io::Result<()> {
    thread::Builder::new()
        .name("rfb-writer".to_owned())
        .spawn(move || writer.run())?;
    Ok(())
}
