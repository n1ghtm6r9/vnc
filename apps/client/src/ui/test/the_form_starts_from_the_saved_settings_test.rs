use crate::{ConnectForm, Settings};

#[test]
fn test_the_form_starts_from_the_saved_settings() {
    let settings = Settings {
        host: "192.168.0.220".to_string(),
        port: 5901,
        username: "bogdan".to_string(),
        remember_password: false,
        apple_remote_desktop: false,
        high_performance: true,
        retina: false,
        ..Settings::default()
    };
    let form = ConnectForm::from_settings(&settings);
    assert_eq!(form.host, "192.168.0.220", "the last address is filled in");
    assert_eq!(form.port, "5901", "the last port is filled in");
    assert_eq!(form.username, "bogdan", "the last user is filled in");
    assert!(form.password.is_empty(), "the password never comes from the settings file");
    assert!(!form.remember_password, "the remember choice is kept");
    assert!(!form.apple_remote_desktop, "the Apple Remote Desktop choice is kept");
    assert!(form.high_performance, "the High Performance choice is kept");
    assert!(!form.retina, "the Retina choice is kept");
}
