use crate::AppleDisplayGeometry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerMessage {
    FramebufferUpdate { rect_count: u16 },
    SetColourMapEntries { first_colour: u16, colours: Vec<[u16; 3]> },
    Bell,
    ServerCutText { text: Vec<u8> },
    OversizedCutText { length: u32 },
    AppleAck,
    AppleNop,
    AppleStateChange { status: u16 },
    AppleClipboard { session_id: u32, text: Option<String> },
    AppleDrag,
    AppleDisplayInfo { size: Option<(u16, u16)>, geometry: Option<AppleDisplayGeometry> },
}
