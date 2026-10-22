use std::io::{self, Write};

pub fn write_client_init(writer: &mut impl Write, shared: bool) -> io::Result<()> {
    writer.write_all(&[u8::from(shared)])
}
