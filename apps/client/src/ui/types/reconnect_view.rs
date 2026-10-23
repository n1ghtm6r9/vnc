pub struct ReconnectView {
    pub heading: &'static str,
    pub reason: String,
    pub seconds_left: Option<u64>,
}
