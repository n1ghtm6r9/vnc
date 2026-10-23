use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpu_scaler::GpuScaler;
use platform::KeyboardGrab;
use rfb_session::{AppleDisplayGeometry, ConnectOptions, CursorShape, SessionStats};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event_loop::EventLoopProxy;
use winit::window::Window;

use super::{AppEvent, LinkState};
use crate::{
    ClipboardSync, ConnectForm, EguiLayer, GpuContext, InputRouter, Language, Notice, Phrase, RemoteCursor, SessionSlot,
    Settings, Toolbar,
};

pub struct ClientApp {
    pub(super) settings: Settings,
    pub(super) language: Language,
    pub(super) settings_path: Option<PathBuf>,
    pub(super) proxy: EventLoopProxy<AppEvent>,
    pub(super) window: Option<Arc<Window>>,
    pub(super) gpu: Option<GpuContext>,
    pub(super) egui: Option<EguiLayer>,
    pub(super) scaler: Option<GpuScaler>,
    pub(super) connect_form: ConnectForm,
    pub(super) toolbar: Toolbar,
    pub(super) session: SessionSlot,
    pub(super) link: LinkState,
    pub(super) link_error: Option<Notice>,
    pub(super) link_heading: Phrase,
    pub(super) last_options: Option<ConnectOptions>,
    pub(super) remote: Option<PhysicalSize<u16>>,
    pub(super) media_picture: Option<PhysicalSize<u16>>,
    pub(super) mac_display: Option<AppleDisplayGeometry>,
    pub(super) stats: Option<SessionStats>,
    pub(super) clipboard: ClipboardSync,
    pub(super) router: InputRouter,
    pub(super) keyboard_grab: Option<KeyboardGrab>,
    pub(super) pointer: Option<PhysicalPosition<f64>>,
    pub(super) pointer_inside: bool,
    pub(super) cursor_shape: Option<CursorShape>,
    pub(super) remote_cursor: Option<RemoteCursor>,
    pub(super) cursor_scale: f64,
    pub(super) cursor_on_remote: bool,
    pub(super) mac_cursor_hidden: bool,
    pub(super) repaint_at: Option<Instant>,
    pub(super) display_resize_at: Option<Instant>,
    pub(super) last_frame_at: Option<Instant>,
    pub(super) frame_interval: Duration,
    pub(super) failed: bool,
}
