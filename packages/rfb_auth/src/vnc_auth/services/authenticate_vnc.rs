use std::io::{Read, Write};

use log::debug;

use super::vnc_auth_response;
use crate::AuthError;

pub fn authenticate_vnc(stream: &mut (impl Read + Write), password: &str) -> Result<(), AuthError> {
    let mut challenge = [0u8; 16];
    stream.read_exact(&mut challenge)?;
    stream.write_all(&vnc_auth_response(&challenge, password))?;
    stream.flush()?;
    debug!("[AUTH] vnc: the challenge is answered");
    Ok(())
}
