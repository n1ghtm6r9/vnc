use rfb::{PixelFormat, ServerInit};

use crate::test_support::SERVER_NAME;

pub fn server_init(width: u16, height: u16) -> ServerInit {
    ServerInit {
        width,
        height,
        pixel_format: PixelFormat::rgba32(),
        name: SERVER_NAME.to_owned(),
    }
}
