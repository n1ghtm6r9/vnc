use crate::{TIGHT_RGB_BYTES, undo_gradient_row};

pub(crate) fn undo_gradient(data: &mut [u8], width: usize) {
    let row_bytes = width * TIGHT_RGB_BYTES;
    if row_bytes == 0 {
        return;
    }
    let mut above: Option<&mut [u8]> = None;
    for row in data.chunks_exact_mut(row_bytes) {
        undo_gradient_row(row, above.as_deref());
        above = Some(row);
    }
}
