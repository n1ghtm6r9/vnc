use platform::GrabbedKey;
use rfb_session::ClientCommand;

use crate::{InputRouter, Settings};

#[test]
fn test_a_grabbed_windows_key_reaches_the_mac_as_cmd() {
    let mut router = InputRouter::new(Settings::default().key_mapping());

    let press = router.grabbed_key(GrabbedKey { virtual_key: 0x5b, down: true });
    let release = router.grabbed_key(GrabbedKey { virtual_key: 0x5b, down: false });
    let other = router.grabbed_key(GrabbedKey { virtual_key: 0x41, down: true });

    assert_eq!(
        press,
        Some(ClientCommand::Key { keysym: 0xffeb, down: true }),
        "pressing the Windows key presses Cmd on the Mac"
    );
    assert_eq!(
        release,
        Some(ClientCommand::Key { keysym: 0xffeb, down: false }),
        "letting go of the Windows key lets go of Cmd"
    );
    assert_eq!(other, None, "only the Windows keys come through the grab");
}
