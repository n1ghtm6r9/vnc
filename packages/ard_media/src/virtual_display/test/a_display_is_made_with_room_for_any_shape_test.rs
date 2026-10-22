use crate::{virtual_display_for, virtual_display_message};

#[test]
fn test_a_display_is_made_with_room_for_any_shape() {
    for (width, height, scale) in [
        (1600, 900, 2),
        (1618, 762, 1),
        (2000, 600, 2),
        (1200, 1600, 2),
    ] {
        let message = virtual_display_message(&[virtual_display_for(width, height, scale)]);
        let word = |offset: usize| {
            u32::from_be_bytes(message[offset..offset + 4].try_into().expect("four bytes"))
        };
        assert_eq!(
            [word(12 + 138), word(12 + 142)],
            [7680, 4320],
            "{width}x{height}@{scale}: the display is made with the room of an 8K one"
        );
    }
    let message = virtual_display_message(&[virtual_display_for(1600, 900, 2)]);
    let word = |offset: usize| {
        u32::from_be_bytes(message[offset..offset + 4].try_into().expect("four bytes"))
    };
    assert_eq!(message.len(), 12 + 156 + 28, "one display of one mode");
    assert_eq!(
        u16::from_be_bytes([message[2], message[3]]) as usize,
        message.len() - 4,
        "the length counts what follows it"
    );
    assert_eq!(
        u16::from_be_bytes([message[12 + 154], message[12 + 155]]),
        1,
        "the record lists one mode"
    );
    assert_eq!(
        [
            word(12 + 156),
            word(12 + 160),
            word(12 + 164),
            word(12 + 168)
        ],
        [3200, 1800, 1600, 900],
        "the mode gives its pixels then its points"
    );
}
