use super::fill_random;

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    fill_random(&mut bytes).expect("the system random generator is unavailable");
    bytes
}
