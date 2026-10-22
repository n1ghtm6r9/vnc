use egui::Rect;

#[derive(Debug, Default)]
pub struct Toolbar {
    pub(super) visible: bool,
    pub(super) rect: Option<Rect>,
}
