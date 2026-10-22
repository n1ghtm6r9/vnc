use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
};

use crate::{MAX_DATAGRAM, MediaChannel, MediaInput, MediaLink};

pub(crate) fn receive_datagrams(
    channel: MediaChannel,
    link: &MediaLink,
    inputs: &Sender<MediaInput>,
    stop: &AtomicBool,
) {
    let mut buffer = vec![0; MAX_DATAGRAM];
    while !stop.load(Ordering::Relaxed) {
        match link.receive(channel, &mut buffer) {
            Ok(Some(size)) => {
                let datagram = MediaInput::Datagram(channel, buffer[..size].to_vec());
                if inputs.send(datagram).is_err() {
                    return;
                }
            }
            Ok(None) => {}
            Err(error) => {
                let reason = format!("the {} port failed: {error}", channel.name());
                let _ = inputs.send(MediaInput::LinkFailed(reason));
                return;
            }
        }
    }
}
