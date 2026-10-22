use crate::mac_line_ends;

#[test]
fn test_windows_text_gets_mac_line_ends() {
    let text = mac_line_ends("first\r\nsecond\nthird\r\n");

    assert_eq!(text, "first\nsecond\nthird\n", "every Windows line end becomes the single line feed the Mac uses");
}
