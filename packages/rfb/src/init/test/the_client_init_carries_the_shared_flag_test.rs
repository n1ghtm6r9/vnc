use crate::{read_client_init, write_client_init};

#[test]
fn test_the_client_init_carries_the_shared_flag() {
    let mut bytes = Vec::new();
    write_client_init(&mut bytes, true).unwrap();
    assert_eq!(bytes, vec![1], "shared is a single non-zero byte");
    assert!(read_client_init(&mut &bytes[..]).unwrap(), "the server side reads the flag back");
}
