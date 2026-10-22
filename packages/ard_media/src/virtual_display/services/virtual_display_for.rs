use crate::{
    DisplayMode, MAX_DISPLAY_PIXELS, MIN_DISPLAY_HEIGHT, MIN_DISPLAY_HEIGHT_HIDPI,
    MIN_DISPLAY_WIDTH, ROOM_PIXEL_HEIGHT, ROOM_PIXEL_WIDTH, VIRTUAL_DISPLAY_NAME, VirtualDisplay,
};

pub fn virtual_display_for(width: u32, height: u32, scale: u32) -> VirtualDisplay {
    let width = f64::from(width.max(1));
    let height = f64::from(height.max(1));
    let factor = f64::from(scale.max(1));
    let max_pixels = f64::from(MAX_DISPLAY_PIXELS);
    let room_width = f64::from(ROOM_PIXEL_WIDTH);
    let room_height = f64::from(ROOM_PIXEL_HEIGHT);
    let even = |value: f64| ((value / 2.0).round() * 2.0).max(2.0);
    let within = |w: f64, h: f64| {
        let (pixel_width, pixel_height) = (even(w * factor), even(h * factor));
        pixel_width * pixel_height <= max_pixels
            && pixel_width <= room_width
            && pixel_height <= room_height
    };
    let min_height = if scale > 1 {
        MIN_DISPLAY_HEIGHT_HIDPI
    } else {
        MIN_DISPLAY_HEIGHT
    };
    let grow = 1f64
        .max(f64::from(MIN_DISPLAY_WIDTH) / width)
        .max(f64::from(min_height) / height);
    let grown_width = width * grow * factor;
    let grown_height = height * grow * factor;
    let fit = 1f64
        .min((max_pixels / (grown_width * grown_height)).sqrt())
        .min(room_width / grown_width)
        .min(room_height / grown_height);
    let mut w = even(width * grow * fit);
    let mut h = even(height * grow * fit);
    while !within(w, h) && (w > 2.0 || h > 2.0) {
        if w >= h {
            w -= 2.0;
        } else {
            h -= 2.0;
        }
    }
    VirtualDisplay {
        name: VIRTUAL_DISPLAY_NAME.into(),
        largest_pixel_width: ROOM_PIXEL_WIDTH,
        largest_pixel_height: ROOM_PIXEL_HEIGHT,
        modes: vec![DisplayMode {
            pixel_width: even(w * factor) as u32,
            pixel_height: even(h * factor) as u32,
            width: w as u32,
            height: h as u32,
            refresh: 60.0,
        }],
    }
}
