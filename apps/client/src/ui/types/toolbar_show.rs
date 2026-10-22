use egui::{Align2, Area, Frame, Id, Order, Ui, Vec2};

use super::{OverlayAction, Toolbar, ToolbarView};
use crate::{paint_toolbar_handle, toolbar_buttons, toolbar_visible};

impl Toolbar {
    pub fn show(&mut self, ui: &mut Ui, view: &ToolbarView) -> Vec<OverlayAction> {
        let ctx = ui.ctx().clone();
        let pointer = ctx.input(|input| input.pointer.hover_pos());
        self.visible = toolbar_visible(self.visible, pointer, self.rect);
        if !self.visible {
            self.rect = None;
            paint_toolbar_handle(&ctx);
            return Vec::new();
        }
        let mut actions = Vec::new();
        let shown = Area::new(Id::unique("toolbar"))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 6.0))
            .show(&ctx, |ui| {
                Frame::popup(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| toolbar_buttons(ui, view, &mut actions));
                });
            });
        self.rect = Some(shown.response.rect);
        actions
    }
}
