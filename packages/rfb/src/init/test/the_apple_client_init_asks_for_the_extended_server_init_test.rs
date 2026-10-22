use crate::write_apple_client_init;

#[test]
fn test_the_apple_client_init_asks_for_the_extended_server_init() {
    let mut bytes = Vec::new();
    write_apple_client_init(&mut bytes).unwrap();
    assert_eq!(
        bytes,
        vec![0xc1],
        "0xC1 tells the Mac the client speaks Apple Remote Desktop and shares the screen"
    );
}
