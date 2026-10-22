use egui::{Key, TextEdit, Ui};

use crate::FIELD_WIDTH;

pub fn labeled_field(ui: &mut Ui, label: &str, value: &mut String, password: bool) -> bool {
    ui.label(label);
    let response = ui.add(TextEdit::singleline(value).password(password).desired_width(FIELD_WIDTH));
    ui.end_row();
    response.lost_focus() && ui.input(|input| input.key_pressed(Key::Enter))
}
