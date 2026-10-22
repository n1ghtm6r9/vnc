pub struct ConnectForm {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub remember_password: bool,
    pub apple_remote_desktop: bool,
    pub high_performance: bool,
    pub retina: bool,
    pub busy: bool,
    pub error: Option<String>,
}
