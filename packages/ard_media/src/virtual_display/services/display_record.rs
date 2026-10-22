use crate::{DISPLAY_MODE_LENGTH, DISPLAY_NAME_LENGTH, POINTS_PER_INCH, VirtualDisplay};

pub(crate) fn display_record(display: &VirtualDisplay) -> Vec<u8> {
    let length = 2 + DISPLAY_NAME_LENGTH + 26 + display.modes.len() * DISPLAY_MODE_LENGTH;
    let mut record = Vec::with_capacity(length);
    record.extend_from_slice(&(length as u16).to_be_bytes());
    let mut name = [0u8; DISPLAY_NAME_LENGTH];
    let text = display.name.as_bytes();
    let kept = text.len().min(DISPLAY_NAME_LENGTH - 1);
    name[..kept].copy_from_slice(&text[..kept]);
    record.extend_from_slice(&name);
    let millimetres = |points: u32| (f64::from(points) * 25.4 / POINTS_PER_INCH) as f32;
    let first = display.modes.first();
    record.extend_from_slice(
        &first
            .map_or(0.0, |mode| millimetres(mode.width))
            .to_be_bytes(),
    );
    record.extend_from_slice(
        &first
            .map_or(0.0, |mode| millimetres(mode.height))
            .to_be_bytes(),
    );
    let largest_width = display
        .modes
        .iter()
        .map(|mode| mode.pixel_width)
        .fold(display.largest_pixel_width, u32::max);
    let largest_height = display
        .modes
        .iter()
        .map(|mode| mode.pixel_height)
        .fold(display.largest_pixel_height, u32::max);
    record.extend_from_slice(&largest_width.to_be_bytes());
    record.extend_from_slice(&largest_height.to_be_bytes());
    record.extend_from_slice(&[0; 6]);
    record.extend_from_slice(&7u16.to_be_bytes());
    record.extend_from_slice(&(display.modes.len() as u16).to_be_bytes());
    for mode in &display.modes {
        record.extend_from_slice(&mode.pixel_width.to_be_bytes());
        record.extend_from_slice(&mode.pixel_height.to_be_bytes());
        record.extend_from_slice(&mode.width.to_be_bytes());
        record.extend_from_slice(&mode.height.to_be_bytes());
        record.extend_from_slice(&mode.refresh.to_be_bytes());
        record.extend_from_slice(&[0; 4]);
    }
    record
}
