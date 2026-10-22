use egui::{Pos2, Rect};

use crate::{TOOLBAR_HIDE_MARGIN, TOOLBAR_REVEAL_EDGE};

pub fn toolbar_visible(was_visible: bool, pointer: Option<Pos2>, toolbar: Option<Rect>) -> bool {
    let Some(pointer) = pointer else {
        return false;
    };
    if pointer.y <= TOOLBAR_REVEAL_EDGE {
        return true;
    }
    was_visible && toolbar.is_some_and(|rect| rect.expand(TOOLBAR_HIDE_MARGIN).contains(pointer))
}
