use crate::{ClientCommand, Outgoing, coalesce_pointer_moves};

#[test]
fn test_a_drag_keeps_the_press_the_last_move_and_the_release() {
    let pointer =
        |x: u16, buttons: u8| Outgoing::Command(ClientCommand::Pointer { x, y: x, buttons });
    let drag = vec![
        pointer(1, 1),
        pointer(2, 1),
        pointer(3, 1),
        pointer(4, 1),
        pointer(4, 0),
    ];

    let sent = coalesce_pointer_moves(drag, 0);

    assert_eq!(
        sent,
        vec![pointer(1, 1), pointer(4, 1), pointer(4, 0)],
        "a fast drag keeps its press, its last move with the button held and its release"
    );
}
