use crate::windows_line_ends;

#[test]
fn test_mac_text_gets_windows_line_ends() {
    assert_eq!(
        windows_line_ends("one\ntwo\r\nthree\n"),
        "one\r\ntwo\r\nthree\r\n",
        "every line of Mac text ends with CRLF on Windows, and lines that already do are left alone"
    );
}
