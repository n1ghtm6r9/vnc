use super::next_random;

pub fn random_bytes(state: &mut u64, length: usize) -> Vec<u8> {
    (0..length)
        .map(|_| next_random(state).to_le_bytes()[3])
        .collect()
}
