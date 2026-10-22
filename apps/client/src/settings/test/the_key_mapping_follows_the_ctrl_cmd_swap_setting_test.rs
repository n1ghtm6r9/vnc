use remote_input::{CommandKeysym, KeyMapping};

use crate::Settings;

#[test]
fn test_the_key_mapping_follows_the_ctrl_cmd_swap_setting() {
    let settings = Settings {
        swap_control_and_command: true,
        ..Settings::default()
    };

    let mapping = settings.key_mapping();

    assert_eq!(
        mapping,
        KeyMapping {
            swap_control_and_command: true,
            command_keysym: CommandKeysym::Super,
        },
        "the saved Ctrl and Cmd swap reaches the keyboard, and Cmd goes out as Super"
    );
}
