use egui::{Align2, Button, Checkbox, Grid, Ui, Vec2, Window};

use super::{ConnectForm, ConnectRequest};
use crate::{
    APPLE_REMOTE_DESKTOP_LABEL, CONNECT_BUTTON, CONNECT_HEADING, CONNECTING_STATUS, FIELD_WIDTH, HIGH_PERFORMANCE_LABEL, HOST_LABEL,
    PASSWORD_LABEL, PORT_LABEL, REMEMBER_PASSWORD_LABEL, RETINA_LABEL, USERNAME_LABEL, labeled_field,
};

impl ConnectForm {
    pub fn show(&mut self, ui: &mut Ui) -> Option<ConnectRequest> {
        let mut submitted = false;
        Window::new(CONNECT_HEADING)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .show(ui.ctx(), |ui| {
                ui.add_enabled_ui(!self.busy, |ui| {
                    Grid::new("connect_form")
                        .num_columns(2)
                        .spacing([12.0, 10.0])
                        .show(ui, |ui| {
                            submitted |= labeled_field(ui, HOST_LABEL, &mut self.host, false);
                            submitted |= labeled_field(ui, PORT_LABEL, &mut self.port, false);
                            submitted |= labeled_field(ui, USERNAME_LABEL, &mut self.username, false);
                            submitted |= labeled_field(ui, PASSWORD_LABEL, &mut self.password, true);
                        });
                    ui.add_space(6.0);
                    ui.checkbox(&mut self.remember_password, REMEMBER_PASSWORD_LABEL);
                    ui.checkbox(&mut self.apple_remote_desktop, APPLE_REMOTE_DESKTOP_LABEL);
                    ui.add_enabled(
                        self.apple_remote_desktop,
                        Checkbox::new(&mut self.high_performance, HIGH_PERFORMANCE_LABEL),
                    );
                    ui.add_enabled(
                        self.apple_remote_desktop && self.high_performance,
                        Checkbox::new(&mut self.retina, RETINA_LABEL),
                    );
                    ui.add_space(10.0);
                    submitted |= ui
                        .add_sized([FIELD_WIDTH, 30.0], Button::new(CONNECT_BUTTON))
                        .clicked();
                });
                if self.busy {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(CONNECTING_STATUS);
                    });
                }
                if let Some(error) = &self.error {
                    ui.add_space(6.0);
                    ui.colored_label(ui.visuals().error_fg_color, error.as_str());
                }
            });
        if !submitted || self.busy {
            return None;
        }
        match self.validate() {
            Ok(request) => {
                self.error = None;
                Some(request)
            }
            Err(message) => {
                self.error = Some(message.to_string());
                None
            }
        }
    }
}
