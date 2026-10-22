use std::{io, net::SocketAddr};

use super::MediaLink;
use crate::{MediaOptions, bind_media_socket};

impl MediaLink {
    pub(crate) fn bind(options: &MediaOptions) -> io::Result<MediaLink> {
        let [audio, video] = options.ports.local;
        Ok(MediaLink {
            sockets: [
                bind_media_socket(options.server, audio)?,
                bind_media_socket(options.server, video)?,
            ],
            remote: options
                .ports
                .remote
                .map(|port| SocketAddr::new(options.server, port)),
        })
    }
}
