use super::SessionEvent;

pub(crate) type EventCallback = Box<dyn FnMut(SessionEvent) + Send>;
