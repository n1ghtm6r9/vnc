use hp_media::MediaEvent;

pub(crate) type MediaCallback = Box<dyn FnMut(MediaEvent) + Send>;
