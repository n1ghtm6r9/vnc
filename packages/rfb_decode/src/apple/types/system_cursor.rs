#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemCursor {
    Default,
    Text,
    Crosshair,
    Grabbing,
    Grab,
    Pointer,
    Wait,
    Help,
    NotAllowed,
    Move,
    NsResize,
    EwResize,
}
