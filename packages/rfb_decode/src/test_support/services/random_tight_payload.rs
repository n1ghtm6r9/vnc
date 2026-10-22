use rfb::Rect;

use super::{compact_length, random_below, random_bytes, random_tight_data};

pub fn random_tight_payload(state: &mut u64, rect: Rect) -> Vec<u8> {
    let area = rect.area();
    let resets = u8::try_from(random_below(state, 16)).expect("a nibble fits a byte");
    match random_below(state, 7) {
        0 => [vec![0x80 | resets], random_bytes(state, 3)].concat(),
        1 => [vec![resets], random_tight_data(state, area * 3)].concat(),
        2 => {
            let colour_count =
                u8::try_from(random_below(state, 256)).expect("a small number fits a byte");
            let colours = usize::from(colour_count) + 1;
            let index_bytes = if colours == 2 {
                usize::from(rect.width).div_ceil(8) * usize::from(rect.height)
            } else {
                area
            };
            [
                vec![0x40 | resets, 1, colour_count],
                random_bytes(state, colours * 3),
                random_tight_data(state, index_bytes),
            ]
            .concat()
        }
        3 => [vec![0x40 | resets, 2], random_tight_data(state, area * 3)].concat(),
        4 => {
            let length = random_below(state, 300);
            [
                vec![0x90 | resets],
                compact_length(length),
                random_bytes(state, length),
            ]
            .concat()
        }
        5 => {
            let length = random_below(state, 300);
            random_bytes(state, length)
        }
        _ => {
            let filter = u8::try_from(random_below(state, 6)).expect("a small number fits a byte");
            [vec![0x40 | resets, filter], random_bytes(state, 40)].concat()
        }
    }
}
