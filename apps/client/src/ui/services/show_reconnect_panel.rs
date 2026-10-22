use egui::{Align2, Ui, Vec2, Window};

use crate::{
    CANCEL_BUTTON, COUNTDOWN_REPAINT_INTERVAL, LINK_LOST_HEADING, OverlayAction, RECONNECT_IN_STATUS,
    RECONNECTING_STATUS, ReconnectView, SECONDS_UNIT,
};

pub fn show_reconnect_panel(ui: &mut Ui, view: &ReconnectView) -> Vec<OverlayAction> {
    let mut actions = Vec::new();
    Window::new(LINK_LOST_HEADING)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ui.ctx(), |ui| {
            if !view.reason.is_empty() {
                ui.label(view.reason.as_str());
                ui.add_space(6.0);
            }
            match view.seconds_left {
                Some(seconds) => {
                    ui.label(format!("{RECONNECT_IN_STATUS} {seconds} {SECONDS_UNIT}"));
                    ui.ctx().request_repaint_after(COUNTDOWN_REPAINT_INTERVAL);
                }
                None => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RECONNECTING_STATUS);
                    });
                }
            }
            ui.add_space(10.0);
            if ui.button(CANCEL_BUTTON).clicked() {
                actions.push(OverlayAction::CancelReconnect);
            }
        });
    actions
}
