use rfb::ProtocolVersion;

use crate::ConnectOptions;

pub(crate) fn choose_version(
    offered: ProtocolVersion,
    options: &ConnectOptions,
) -> ProtocolVersion {
    let apple = options.apple_remote_desktop && !options.credentials.username.is_empty();
    match offered {
        ProtocolVersion::AppleRemoteDesktop if !apple => ProtocolVersion::V3_8,
        version => version,
    }
}
