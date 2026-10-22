pub fn length_prefixed(data: &[u8]) -> Vec<u8> {
    let length = u32::try_from(data.len()).expect("a test payload fits a u32 length");
    let mut framed = length.to_be_bytes().to_vec();
    framed.extend_from_slice(data);
    framed
}
