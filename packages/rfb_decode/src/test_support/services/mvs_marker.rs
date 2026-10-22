pub fn mvs_marker(text: &str) -> String {
    text.bytes().map(|byte| format!("{byte:08b}")).collect()
}
