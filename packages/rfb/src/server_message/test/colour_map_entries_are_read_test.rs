use crate::ServerMessage;

#[test]
fn test_colour_map_entries_are_read() {
    let bytes = [1u8, 0, 0, 7, 0, 1, 0xff, 0xff, 0, 0, 0x80, 0x00];
    let message = ServerMessage::read(&mut &bytes[..]).unwrap();
    assert_eq!(
        message,
        ServerMessage::SetColourMapEntries { first_colour: 7, colours: vec![[0xffff, 0, 0x8000]] },
        "type, padding, first colour, count, then 16-bit red, green, blue"
    );
}
