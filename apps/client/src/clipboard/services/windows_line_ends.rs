pub fn windows_line_ends(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\n', "\r\n")
}
