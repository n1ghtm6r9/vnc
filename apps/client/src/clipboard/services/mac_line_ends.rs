pub fn mac_line_ends(text: &str) -> String {
    text.replace("\r\n", "\n")
}
