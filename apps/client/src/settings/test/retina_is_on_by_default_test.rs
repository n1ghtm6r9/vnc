use crate::Settings;

#[test]
fn test_retina_is_on_by_default() {
    assert!(
        Settings::default().retina,
        "High Performance asks the Mac for a Retina display unless the user turns it off"
    );
}
