use log::info;
use rfb_session::{ClientCommand, encodings_for_quality};
use winit::window::Fullscreen;

use super::ClientApp;
use crate::{OverlayAction, session_quality};

impl ClientApp {
    pub(super) fn handle_overlay_action(&mut self, action: OverlayAction) {
        match action {
            OverlayAction::Disconnect | OverlayAction::CancelReconnect => {
                info!("[UI] the session is closed by the user");
                self.end_session(None);
            }
            OverlayAction::ScaleMode(mode) => {
                self.settings.scale_mode = mode;
                self.save_settings();
            }
            OverlayAction::Quality(quality) => {
                self.settings.quality = quality;
                self.save_settings();
                let encodings = encodings_for_quality(session_quality(quality));
                if let Some(options) = &mut self.last_options {
                    options.encodings = encodings.clone();
                }
                self.session.send(ClientCommand::SetEncodings(encodings));
            }
            OverlayAction::Sound(place) => {
                self.settings.sound = place;
                self.save_settings();
                self.change_sound(place);
            }
            OverlayAction::Language(language) => {
                self.language = language;
                self.settings.language = Some(language);
                self.save_settings();
            }
            OverlayAction::SwapControlAndCommand(swap) => {
                self.settings.swap_control_and_command = swap;
                self.save_settings();
                for command in self.router.set_mapping(self.settings.key_mapping()) {
                    self.session.send(command);
                }
            }
            OverlayAction::Fullscreen(fullscreen) => {
                if let Some(window) = &self.window {
                    window.set_fullscreen(fullscreen.then_some(Fullscreen::Borderless(None)));
                }
            }
            OverlayAction::Refresh => self.session.send(ClientCommand::RefreshScreen),
        }
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
