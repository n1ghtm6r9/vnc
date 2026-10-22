use crate::{ProtocolVersion, read_security_types, write_security_types};

#[test]
fn test_the_apple_type_list_is_read_whole() {
    let mut bytes = Vec::new();
    write_security_types(&mut bytes, &[30, 33, 36, 31, 32, 35]).unwrap();
    let types = read_security_types(&mut &bytes[..], ProtocolVersion::V3_8).unwrap();
    assert_eq!(types, vec![30, 33, 36, 31, 32, 35], "the list the Mac offered on 2026-10-21 reads back in its order");
}
