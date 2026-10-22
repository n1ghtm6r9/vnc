use crate::credential_target;

#[test]
fn test_the_credential_target_names_user_host_and_port() {
    assert_eq!(
        credential_target("bogdan", "192.168.0.220", 5900),
        "vnc:bogdan@192.168.0.220:5900",
        "each Mac account gets its own Credential Manager entry"
    );
}
