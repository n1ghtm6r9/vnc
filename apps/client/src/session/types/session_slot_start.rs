use rfb_session::{ConnectOptions, start_session};

use super::SessionSlot;
use crate::AppEvent;

impl SessionSlot {
    pub fn start(&mut self, options: ConnectOptions, sink: impl Fn(AppEvent) + Send + 'static) {
        self.stop();
        let generation = self.generation;
        let on_event = move |event| sink(AppEvent::Session { generation, event });
        self.handle = Some(start_session(options, Box::new(on_event)));
    }
}
