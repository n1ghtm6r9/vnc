pub(crate) fn vnc_des_key(password: &str) -> [u8; 8] {
    let mut key = [0u8; 8];
    for (key_byte, password_byte) in key.iter_mut().zip(password.bytes()) {
        *key_byte = password_byte.reverse_bits();
    }
    key
}
