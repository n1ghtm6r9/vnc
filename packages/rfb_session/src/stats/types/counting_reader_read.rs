use std::io::{self, Read};

use super::CountingReader;

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.inner.read(buf)?;
        self.count += read as u64;
        Ok(read)
    }
}
