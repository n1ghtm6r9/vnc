use crate::TIGHT_RGB_BYTES;

pub(crate) fn undo_gradient_row(row: &mut [u8], above: Option<&[u8]>) {
    let above_value = |index: usize| {
        above
            .and_then(|above| above.get(index))
            .map_or(0, |&value| i16::from(value))
    };
    for index in 0..row.len() {
        let (left, above_left) = match index.checked_sub(TIGHT_RGB_BYTES) {
            Some(previous) => (i16::from(row[previous]), above_value(previous)),
            None => (0, 0),
        };
        let predicted = (left + above_value(index) - above_left).clamp(0, i16::from(u8::MAX));
        row[index] = row[index].wrapping_add(u8::try_from(predicted).unwrap_or(u8::MAX));
    }
}
