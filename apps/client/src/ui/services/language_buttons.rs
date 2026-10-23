use egui::Ui;

use crate::{LANGUAGE_CHOICES, Language, OverlayAction};

pub fn language_buttons(ui: &mut Ui, current: Language, actions: &mut Vec<OverlayAction>) {
    for (language, label, name) in LANGUAGE_CHOICES {
        if ui.selectable_label(current == language, label).on_hover_text(name).clicked() && language != current {
            actions.push(OverlayAction::Language(language));
        }
    }
}
