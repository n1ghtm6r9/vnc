use super::SystemCursor;

impl SystemCursor {
    pub fn from_apple_id(id: u32) -> SystemCursor {
        match id {
            1 => SystemCursor::Text,
            2 => SystemCursor::Crosshair,
            3 => SystemCursor::Grabbing,
            4 => SystemCursor::Grab,
            5 => SystemCursor::Pointer,
            6 => SystemCursor::Wait,
            7 => SystemCursor::Help,
            8 => SystemCursor::NotAllowed,
            9 => SystemCursor::Move,
            10 => SystemCursor::NsResize,
            11 => SystemCursor::EwResize,
            _ => SystemCursor::Default,
        }
    }
}
