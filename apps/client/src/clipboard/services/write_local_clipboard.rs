pub fn write_local_clipboard(text: &str) -> Result<(), arboard::Error> {
    arboard::Clipboard::new()?.set_text(text)
}
