use std::io::{self, Write};

use super::apple_scroll_lines;
use crate::{APPLE_SCROLL_PIXELS_PER_LINE, AppleScroll, CLIENT_APPLE_GESTURE};

pub fn write_apple_scroll(writer: &mut impl Write, scroll: AppleScroll) -> io::Result<()> {
    let lines_x = f64::from(scroll.right) / APPLE_SCROLL_PIXELS_PER_LINE;
    let lines_y = f64::from(scroll.down) / APPLE_SCROLL_PIXELS_PER_LINE;
    let fixed = |lines: f64| (lines * 65536.0).round() as i32;
    let mut bytes = Vec::with_capacity(94);
    bytes.extend_from_slice(&[CLIENT_APPLE_GESTURE, 0, 0, 32, 0, 2, 0, 8]);
    bytes.extend_from_slice(&(lines_x as f32).to_be_bytes());
    bytes.extend_from_slice(&(lines_y as f32).to_be_bytes());
    bytes.extend_from_slice(&0f32.to_be_bytes());
    bytes.extend_from_slice(&1u32.to_be_bytes());
    bytes.extend_from_slice(&[0; 8]);
    bytes.extend_from_slice(&scroll.x.to_be_bytes());
    bytes.extend_from_slice(&scroll.y.to_be_bytes());
    bytes.extend_from_slice(&[CLIENT_APPLE_GESTURE, 0, 0, 54, 0, 1, 0, 11]);
    bytes.extend_from_slice(&apple_scroll_lines(scroll.right).to_be_bytes());
    bytes.extend_from_slice(&apple_scroll_lines(scroll.down).to_be_bytes());
    bytes.extend_from_slice(&[0; 2]);
    bytes.extend_from_slice(&fixed(lines_x).to_be_bytes());
    bytes.extend_from_slice(&fixed(lines_y).to_be_bytes());
    bytes.extend_from_slice(&[0; 4]);
    bytes.extend_from_slice(&scroll.right.to_be_bytes());
    bytes.extend_from_slice(&scroll.down.to_be_bytes());
    bytes.extend_from_slice(&[0; 20]);
    bytes.extend_from_slice(&scroll.x.to_be_bytes());
    bytes.extend_from_slice(&scroll.y.to_be_bytes());
    writer.write_all(&bytes)
}
