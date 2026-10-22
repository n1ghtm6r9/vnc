use super::{length_prefixed, random_below, random_bytes};

pub fn random_mvs_payload(state: &mut u64) -> Vec<u8> {
    let length = random_below(state, 300);
    let mut update = random_bytes(state, length);
    let kind = u8::try_from(random_below(state, 3)).expect("an update type fits a byte");
    if let Some(first) = update.first_mut() {
        *first = kind;
    }
    if kind == 0 && update.len() >= 6 {
        let body_offset = 6 + random_below(state, update.len() - 5);
        let offset = u32::try_from(body_offset)
            .expect("a short test payload offset fits a u32")
            .to_be_bytes();
        update[3..6].copy_from_slice(&offset[1..]);
    }
    length_prefixed(&update)
}
