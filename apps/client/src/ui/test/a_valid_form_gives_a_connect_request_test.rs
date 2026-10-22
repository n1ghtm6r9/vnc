use super::valid_form::valid_form;
use crate::ConnectRequest;

#[test]
fn test_a_valid_form_gives_a_connect_request() {
    let request = valid_form().validate().unwrap();
    let expected = ConnectRequest {
        host: "192.168.0.220".to_string(),
        port: 5900,
        username: "bogdan".to_string(),
        password: " пароль ".to_string(),
        remember_password: true,
        apple_remote_desktop: false,
        high_performance: true,
        retina: false,
    };
    assert_eq!(request, expected, "host and user are trimmed, the password is kept exactly as typed");
}
