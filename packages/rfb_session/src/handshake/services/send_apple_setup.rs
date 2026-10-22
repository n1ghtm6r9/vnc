use std::{
    io::{self, BufWriter, Write},
    slice,
};

use ard_media::{VirtualDisplay, virtual_display_message};
use rfb::{
    APPLE_MODE_CONTROL, PixelFormat, Rect, write_apple_auto_pasteboard, write_apple_auto_update,
    write_apple_set_mode, write_apple_show_all_displays, write_apple_viewer_info,
    write_framebuffer_update_request, write_set_encodings, write_set_pixel_format,
};

use crate::{apple_encodings, high_performance_encodings};

pub(crate) fn send_apple_setup(
    stream: &mut impl Write,
    encodings: &[i32],
    screen: Rect,
    high_performance: Option<&VirtualDisplay>,
) -> io::Result<()> {
    let encodings = match high_performance {
        Some(_) => high_performance_encodings(encodings),
        None => apple_encodings(encodings),
    };
    let mut output = BufWriter::new(stream);
    write_apple_viewer_info(&mut output)?;
    write_apple_set_mode(&mut output, APPLE_MODE_CONTROL)?;
    if let Some(display) = high_performance {
        output.write_all(&virtual_display_message(slice::from_ref(display)))?;
    }
    write_apple_show_all_displays(&mut output)?;
    write_apple_auto_pasteboard(&mut output, true)?;
    write_set_encodings(&mut output, &encodings)?;
    write_set_pixel_format(&mut output, &PixelFormat::rgba32())?;
    write_set_encodings(&mut output, &encodings)?;
    write_framebuffer_update_request(&mut output, false, screen)?;
    write_apple_auto_update(&mut output, screen)?;
    output.flush()
}
