use std::io::{self, Read};

pub fn read_bytes(reader: &mut impl Read, len: usize) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::with_capacity(len.min(1 << 24));
    reader.by_ref().take(len as u64).read_to_end(&mut buffer)?;
    if buffer.len() < len {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("the stream ended after {} of {len} bytes", buffer.len()),
        ));
    }
    Ok(buffer)
}
