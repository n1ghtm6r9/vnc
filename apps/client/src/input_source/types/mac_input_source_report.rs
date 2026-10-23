use super::MacInputSource;
use crate::ABC_INPUT_SOURCE;

impl MacInputSource {
    pub fn report(&mut self, source: &str) -> Option<&'static str> {
        if source == ABC_INPUT_SOURCE {
            return None;
        }
        self.own = Some(source.to_owned());
        Some(ABC_INPUT_SOURCE)
    }
}
