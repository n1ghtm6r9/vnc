pub fn rre_payload(background: [u8; 4], subrects: &[Vec<u8>]) -> Vec<u8> {
    let count =
        u32::try_from(subrects.len()).expect("a test payload has fewer subrects than a u32 counts");
    let mut bytes = count.to_be_bytes().to_vec();
    bytes.extend_from_slice(&background);
    for subrect in subrects {
        bytes.extend_from_slice(subrect);
    }
    bytes
}
