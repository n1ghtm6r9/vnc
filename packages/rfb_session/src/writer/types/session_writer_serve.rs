use std::io::{self, Write};

use super::{Outgoing, SessionWriter};
use crate::coalesce_pointer_moves;

impl SessionWriter {
    pub(super) fn serve(&mut self) -> io::Result<()> {
        while let Ok(first) = self.queue.recv() {
            let mut batch = vec![first];
            batch.extend(self.queue.try_iter());
            for message in coalesce_pointer_moves(batch, self.pointer_buttons) {
                if message == Outgoing::Stop {
                    return self.output.flush();
                }
                self.write_message(message)?;
            }
            self.output.flush()?;
        }
        Ok(())
    }
}
