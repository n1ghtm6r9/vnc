use std::sync::mpsc::{self, Receiver};

use crate::MediaEvent;

pub fn event_channel() -> (Box<dyn FnMut(MediaEvent) + Send>, Receiver<MediaEvent>) {
    let (sender, events) = mpsc::channel();
    let on_event = Box::new(move |event| {
        let _ = sender.send(event);
    });
    (on_event, events)
}
