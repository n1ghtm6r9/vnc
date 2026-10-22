use crate::ClipboardSync;

#[test]
fn test_new_local_text_is_sent_once() {
    let mut sync = ClipboardSync::default();
    assert_eq!(sync.take_changed("first".to_string()).as_deref(), Some("first"), "new local text goes to the Mac");
    assert_eq!(sync.take_changed("first".to_string()), None, "the same text is not sent twice");
    assert_eq!(sync.take_changed(String::new()), None, "an empty clipboard sends nothing");
    assert_eq!(sync.take_changed("second".to_string()).as_deref(), Some("second"), "changed text goes again");
}
