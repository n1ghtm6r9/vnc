pub fn credential_target(username: &str, host: &str, port: u16) -> String {
    format!("vnc:{username}@{host}:{port}")
}
