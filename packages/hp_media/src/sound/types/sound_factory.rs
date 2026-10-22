use super::SoundSink;

pub(crate) type SoundFactory = Box<dyn FnMut() -> Box<dyn SoundSink> + Send>;
