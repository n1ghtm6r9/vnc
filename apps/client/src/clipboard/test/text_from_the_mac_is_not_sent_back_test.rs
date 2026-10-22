use crate::ClipboardSync;

#[test]
fn test_text_from_the_mac_is_not_sent_back() {
    let mut sync = ClipboardSync::default();
    sync.note_synced("copied on the Mac");
    assert_eq!(sync.take_changed("copied on the Mac".to_string()), None, "text that came from the Mac is not echoed back");
}
