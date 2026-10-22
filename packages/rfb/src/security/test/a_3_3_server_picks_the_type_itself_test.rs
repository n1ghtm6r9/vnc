use crate::{ProtocolVersion, read_security_types};

#[test]
fn test_a_3_3_server_picks_the_type_itself() {
    let bytes = [0u8, 0, 0, 2];
    let types = read_security_types(&mut &bytes[..], ProtocolVersion::V3_3).unwrap();
    assert_eq!(types, vec![2], "a 3.3 server sends the one type it chose as a u32");
}
