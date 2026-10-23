use std::path::PathBuf;

use winit::event_loop::EventLoopProxy;

use super::{AppEvent, ClientApp, LinkState};
use crate::{
    ClipboardSync, ConnectForm, InputRouter, LINK_LOST_HEADING, SessionSlot, Settings, Toolbar, frame_interval,
    load_remembered_password,
};

impl ClientApp {
    pub fn new(settings: Settings, settings_path: Option<PathBuf>, proxy: EventLoopProxy<AppEvent>) -> ClientApp {
        let mut connect_form = ConnectForm::from_settings(&settings);
        connect_form.password = load_remembered_password(&settings);
        let router = InputRouter::new(settings.key_mapping());
        ClientApp {
            settings,
            settings_path,
            proxy,
            window: None,
            gpu: None,
            egui: None,
            scaler: None,
            connect_form,
            toolbar: Toolbar::default(),
            session: SessionSlot::default(),
            link: LinkState::Offline,
            link_error: None,
            link_heading: LINK_LOST_HEADING,
            last_options: None,
            remote: None,
            media_picture: None,
            mac_display: None,
            stats: None,
            clipboard: ClipboardSync::default(),
            router,
            keyboard_grab: None,
            pointer: None,
            pointer_inside: false,
            cursor_shape: None,
            remote_cursor: None,
            cursor_scale: 1.0,
            cursor_on_remote: false,
            mac_cursor_hidden: false,
            repaint_at: None,
            display_resize_at: None,
            last_frame_at: None,
            frame_interval: frame_interval(None),
            failed: false,
        }
    }
}
