pub fn read_local_clipboard() -> Result<String, arboard::Error> {
    arboard::Clipboard::new()?.get_text()
}
