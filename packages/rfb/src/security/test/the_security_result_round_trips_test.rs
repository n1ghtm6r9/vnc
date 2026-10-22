use crate::{ProtocolVersion, read_security_result, write_security_result};

#[test]
fn test_the_security_result_round_trips() {
    let mut bytes = Vec::new();
    write_security_result(&mut bytes, None).unwrap();
    assert_eq!(bytes, vec![0, 0, 0, 0], "success is a zero u32");
    assert!(read_security_result(&mut &bytes[..], ProtocolVersion::V3_8).is_ok(), "a zero result is a success");
}
