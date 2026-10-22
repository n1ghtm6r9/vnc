use rfb_auth::Credentials;

use crate::choose_security;

#[test]
fn test_the_client_picks_the_first_security_type_it_can_use() {
    let full = Credentials::new("bogdan", "secret");
    let no_user_name = Credentials::new("", "secret");
    let no_password = Credentials::new("bogdan", "");
    let nothing = Credentials::new("", "");
    let cases = [
        (&[30u8, 33, 36, 31, 32, 35][..], &full, 30),
        (&[1, 2, 30][..], &full, 30),
        (&[30, 2][..], &no_user_name, 2),
        (&[2, 1][..], &no_password, 1),
        (&[30, 2, 1][..], &nothing, 1),
    ];
    for (offered, credentials, expected) in cases {
        let chosen = choose_security(offered, credentials).ok();
        assert_eq!(
            chosen,
            Some(expected),
            "with {credentials:?} the client takes type {expected} from {offered:?}"
        );
    }
}
