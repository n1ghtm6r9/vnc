use egui::{Align2, Id, Ui, Vec2, Window};

use crate::{
    CANCEL_BUTTON, COUNTDOWN_REPAINT_INTERVAL, OverlayAction, RECONNECT_IN_STATUS, RECONNECTING_STATUS, ReconnectView,
    SECONDS_UNIT,
};

pub fn show_reconnect_panel(ui: &mut Ui, view: &ReconnectView) -> Vec<OverlayAction> {
    let language = view.language;
    let mut actions = Vec::new();
    Window::new(view.heading.text(language))
        .id(Id::unique("reconnect_window"))
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ui.ctx(), |ui| {
            if let Some(reason) = &view.reason {
                ui.label(reason.text(language));
                ui.add_space(6.0);
            }
            match view.seconds_left {
                Some(seconds) => {
                    let (prefix, unit) = (RECONNECT_IN_STATUS.text(language), SECONDS_UNIT.text(language));
                    ui.label(format!("{prefix} {seconds} {unit}"));
                    ui.ctx().request_repaint_after(COUNTDOWN_REPAINT_INTERVAL);
                }
                None => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RECONNECTING_STATUS.text(language));
                    });
                }
            }
            ui.add_space(10.0);
            if ui.button(CANCEL_BUTTON.text(language)).clicked() {
                actions.push(OverlayAction::CancelReconnect);
            }
        });
    actions
}
