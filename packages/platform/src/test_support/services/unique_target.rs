use std::time::{SystemTime, UNIX_EPOCH};

pub fn unique_target(label: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    format!("vnc-test:{label}:{}:{nanos}", std::process::id())
}
