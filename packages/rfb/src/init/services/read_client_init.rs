use std::io::{self, Read};

use crate::read_u8;

pub fn read_client_init(reader: &mut impl Read) -> io::Result<bool> {
    Ok(read_u8(reader)? != 0)
}
