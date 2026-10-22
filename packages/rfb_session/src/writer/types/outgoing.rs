use rfb::Rect;

use crate::ClientCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Outgoing {
    Command(ClientCommand),
    UpdateRequest { incremental: bool, rect: Rect },
    AutoUpdate { rect: Rect },
    ClipboardSession(u32),
    ClipboardRequest,
    ClipboardResend,
    MediaMessage(Vec<u8>),
    Stop,
}
