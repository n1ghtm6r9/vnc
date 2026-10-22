use std::io::{Read, Write};

use log::debug;
use rfb::{ProtocolVersion, ServerInit, write_client_init};

use super::{choose_version, ensure_supported_desktop, exchange_apple_init, negotiate_security};
use crate::{ConnectOptions, Handshake, SessionError};

pub(crate) fn perform_handshake(
    stream: &mut (impl Read + Write),
    options: &ConnectOptions,
) -> Result<Handshake, SessionError> {
    let offered = ProtocolVersion::read_server(stream)?;
    let version = choose_version(offered, options);
    version.write_client(stream)?;
    debug!("[SESSION] the server speaks {offered:?}, the client answers {version:?}");
    let apple_key = negotiate_security(stream, version, &options.credentials)?;
    let apple_remote_desktop = version == ProtocolVersion::AppleRemoteDesktop;
    let (init, capabilities) = if apple_remote_desktop {
        let apple_init = exchange_apple_init(stream)?;
        (apple_init.init, Some(apple_init.capabilities))
    } else {
        write_client_init(stream, options.shared)?;
        (ServerInit::read(stream)?, None)
    };
    ensure_supported_desktop(init.width, init.height)?;
    Ok(Handshake {
        init,
        apple_key: apple_key.filter(|_| apple_remote_desktop),
        capabilities,
    })
}
