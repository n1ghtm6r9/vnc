pub fn is_irap(nal_type: u8) -> bool {
    (16..=21).contains(&nal_type)
}
