use std::io::{self, Cursor, Read, Write};

#[derive(Default)]
pub(super) struct RecordingStream {
    pub(super) input: Cursor<Vec<u8>>,
    pub(super) writes: Vec<Vec<u8>>,
    pub(super) flushes: usize,
}

impl Read for RecordingStream {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.input.read(buffer)
    }
}

impl Write for RecordingStream {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.writes.push(buffer.to_vec());
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        Ok(())
    }
}
