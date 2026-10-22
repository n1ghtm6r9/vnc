use crate::{ProtocolVersion, read_security_types, write_security_types};

#[test]
fn test_the_apple_version_reads_the_type_list() {
    let mut bytes = Vec::new();
    write_security_types(&mut bytes, &[30, 33]).unwrap();
    let types = read_security_types(&mut &bytes[..], ProtocolVersion::AppleRemoteDesktop).unwrap();
    assert_eq!(types, vec![30, 33], "the Apple version lists its security types the way 3.8 does");
}
