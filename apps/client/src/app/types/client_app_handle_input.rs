use remote_input::character_of_key;
use winit::event::WindowEvent;
use winit::window::Window;

use super::{ClientApp, LinkState};

impl ClientApp {
    pub(super) fn handle_input(&mut self, window: &Window, event: &WindowEvent, consumed: bool) {
        if self.link != LinkState::Online {
            return;
        }
        let Some(remote) = self.remote else {
            return;
        };
        let commands = match event {
            WindowEvent::Focused(false) => self.router.release_all(),
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } if !(consumed && event.state.is_pressed()) => self
                .router
                .key(event.physical_key, character_of_key(&event.logical_key), event.state.is_pressed())
                .into_iter()
                .collect(),
            WindowEvent::MouseInput { state, button, .. } if !(consumed && state.is_pressed()) => self
                .router
                .button(*button, state.is_pressed())
                .into_iter()
                .collect(),
            WindowEvent::MouseWheel { delta, .. } if !consumed => self.router.wheel(*delta),
            WindowEvent::CursorMoved { position, .. } if !consumed => {
                let Some(view) = self.current_view(window) else {
                    return;
                };
                self.router.pointer_moved(*position, view, remote).into_iter().collect()
            }
            _ => return,
        };
        for command in commands {
            self.session.send(command);
        }
    }
}
