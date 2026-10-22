use crate::{ClientMessage, write_apple_show_all_displays};

#[test]
fn test_the_apple_display_choice_shows_all_displays() {
    let mut bytes = Vec::new();
    write_apple_show_all_displays(&mut bytes).unwrap();
    assert_eq!(bytes, vec![0x0d, 1, 0, 0, 0, 0, 0, 0], "SetDisplay with the combine flag and display 0 shows every display");
    assert_eq!(
        ClientMessage::read(&mut &bytes[..]).unwrap(),
        ClientMessage::AppleSetDisplay { combine_all: true, display_id: 0 },
        "the server side reads the choice back"
    );
}
