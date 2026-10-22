use crate::MediaChannel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Datagram {
    pub(crate) channel: MediaChannel,
    pub(crate) bytes: Vec<u8>,
}
