use std::process::ExitCode;

use log::error;
use winit::event_loop::EventLoop;

use crate::{AppEvent, ClientApp, Settings, init_logger, log_file_path, settings_path};

pub fn run() -> ExitCode {
    init_logger(log_file_path().as_deref());
    let settings_path = settings_path();
    let settings = settings_path
        .as_deref()
        .map(Settings::load)
        .unwrap_or_default();
    let event_loop = match EventLoop::<AppEvent>::with_user_event().build() {
        Ok(event_loop) => event_loop,
        Err(error) => {
            error!("[UI] the event loop could not start: {error}");
            return ExitCode::FAILURE;
        }
    };
    let mut app = ClientApp::new(settings, settings_path, event_loop.create_proxy());
    if let Err(error) = event_loop.run_app(&mut app) {
        error!("[UI] the event loop stopped with an error: {error}");
        return ExitCode::FAILURE;
    }
    app.exit_code()
}
