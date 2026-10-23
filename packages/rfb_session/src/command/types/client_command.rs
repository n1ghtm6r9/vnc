#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientCommand {
    Pointer {
        x: u16,
        y: u16,
        buttons: u8,
    },
    Key {
        keysym: u32,
        down: bool,
    },
    Scroll {
        x: u16,
        y: u16,
        right: i32,
        down: i32,
    },
    ClipboardText(String),
    RefreshScreen,
    SetEncodings(Vec<i32>),
    ResizeDisplay {
        width: u32,
        height: u32,
    },
    PlaySound(bool),
    SetInputSource(String),
}
