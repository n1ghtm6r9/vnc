use crate::{ClientCommand, Outgoing};

pub(crate) fn coalesce_pointer_moves(batch: Vec<Outgoing>, buttons_before: u8) -> Vec<Outgoing> {
    let mut kept: Vec<Outgoing> = Vec::with_capacity(batch.len());
    let mut buttons = buttons_before;
    let mut last_kept_is_move = false;
    for message in batch {
        let pointer_buttons = match &message {
            Outgoing::Command(ClientCommand::Pointer {
                buttons: pressed, ..
            }) => Some(*pressed),
            _ => None,
        };
        let is_move = pointer_buttons == Some(buttons);
        if is_move && last_kept_is_move {
            kept.pop();
        }
        if let Some(pressed) = pointer_buttons {
            buttons = pressed;
        }
        last_kept_is_move = is_move;
        kept.push(message);
    }
    kept
}
