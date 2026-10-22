use std::io;

use rfb::write_set_encodings;

use super::SessionWriter;
use crate::{apple_encodings, high_performance_encodings};

impl SessionWriter {
    pub(super) fn write_encodings(&mut self, encodings: &[i32]) -> io::Result<()> {
        if self.apple_key.is_none() {
            return write_set_encodings(&mut self.output, encodings);
        }
        let encodings = match self.streaming {
            Some(_) => high_performance_encodings(encodings),
            None => apple_encodings(encodings),
        };
        write_set_encodings(&mut self.output, &encodings)
    }
}
