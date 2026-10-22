use std::{
    io,
    sync::{Arc, atomic::AtomicBool, mpsc::Sender},
    thread,
};

use super::receive_datagrams;
use crate::{MediaChannel, MediaInput, MediaLink};

pub(crate) fn spawn_receiver(
    channel: MediaChannel,
    link: Arc<MediaLink>,
    inputs: Sender<MediaInput>,
    stop: Arc<AtomicBool>,
) -> io::Result<()> {
    thread::Builder::new()
        .name(format!("hp-{}", channel.name()))
        .spawn(move || receive_datagrams(channel, &link, &inputs, &stop))
        .map(drop)
}
