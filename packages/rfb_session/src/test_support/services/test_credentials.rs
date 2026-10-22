use rfb_auth::Credentials;

pub fn test_credentials() -> Credentials {
    Credentials::new("bogdan", "пароль от мака")
}
