use crate::{DisplayMode, VirtualDisplay, virtual_display_message};

#[test]
fn test_a_display_never_states_a_largest_size_below_a_mode() {
    let display = VirtualDisplay {
        name: "A".into(),
        largest_pixel_width: 1000,
        largest_pixel_height: 3000,
        modes: vec![DisplayMode {
            pixel_width: 2560,
            pixel_height: 1440,
            width: 1280,
            height: 720,
            refresh: 60.0,
        }],
    };

    let message = virtual_display_message(&[display]);
    let word = |offset: usize| {
        u32::from_be_bytes(message[offset..offset + 4].try_into().expect("four bytes"))
    };

    assert_eq!(
        [word(12 + 138), word(12 + 142)],
        [2560, 3000],
        "the largest size is at least that of every mode"
    );
}
