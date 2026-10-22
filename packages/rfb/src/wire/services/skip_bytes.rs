use std::io::{self, Read};

pub fn skip_bytes(reader: &mut impl Read, len: usize) -> io::Result<()> {
    let skipped = io::copy(&mut reader.by_ref().take(len as u64), &mut io::sink())?;
    if skipped < len as u64 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("the stream ended after {skipped} of {len} skipped bytes"),
        ));
    }
    Ok(())
}
