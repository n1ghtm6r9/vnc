use crate::{Settings, remembered_password_target};

#[test]
fn test_a_password_is_looked_up_only_for_a_remembered_user_and_host() {
    let remembered = Settings {
        host: "192.168.0.220".to_string(),
        username: "owner".to_string(),
        ..Settings::default()
    };
    let not_remembered = Settings {
        remember_password: false,
        ..remembered.clone()
    };
    let without_user = Settings {
        username: String::new(),
        ..remembered.clone()
    };
    let without_host = Settings {
        host: String::new(),
        ..remembered.clone()
    };

    assert_eq!(
        remembered_password_target(&remembered).as_deref(),
        Some("vnc:owner@192.168.0.220:5900"),
        "a remembered login is looked up under its user, host and port"
    );
    assert_eq!(remembered_password_target(&not_remembered), None, "nothing is read when the password is not remembered");
    assert_eq!(remembered_password_target(&without_user), None, "nothing is read before a user name is known");
    assert_eq!(remembered_password_target(&without_host), None, "nothing is read before a host is known");
}
