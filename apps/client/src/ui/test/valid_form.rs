use crate::ConnectForm;

pub(super) fn valid_form() -> ConnectForm {
    ConnectForm {
        host: " 192.168.0.220 ".to_string(),
        port: "5900".to_string(),
        username: "bogdan".to_string(),
        password: " пароль ".to_string(),
        remember_password: true,
        apple_remote_desktop: false,
        high_performance: true,
        retina: false,
        busy: false,
        error: None,
    }
}
