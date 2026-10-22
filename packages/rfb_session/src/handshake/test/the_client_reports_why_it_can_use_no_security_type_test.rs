use rfb_auth::Credentials;

use crate::{SessionError, choose_security};

#[test]
fn test_the_client_reports_why_it_can_use_no_security_type() {
    let no_user_name = choose_security(&[30, 33, 36], &Credentials::new("", "secret"));
    let unknown_only = choose_security(&[5, 16], &Credentials::new("bogdan", "secret"));

    assert!(
        matches!(no_user_name, Err(SessionError::MissingCredentials)),
        "an Apple-only server and an empty user name report missing credentials, got {no_user_name:?}"
    );
    assert!(
        matches!(&unknown_only, Err(SessionError::NoSupportedSecurity(offered)) if offered == &[5, 16]),
        "a server with only unknown types reports what it offered, got {unknown_only:?}"
    );
}
