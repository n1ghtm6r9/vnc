use std::io::Read;

use super::HextileDecoder;
use crate::{DecodeTarget, OPAQUE_BLACK};

impl<'a, R: Read> HextileDecoder<'a, R> {
    pub(crate) fn new(reader: &'a mut R, target: DecodeTarget<'a>) -> HextileDecoder<'a, R> {
        HextileDecoder {
            reader,
            converter: target.converter,
            state: target.state,
            framebuffer: target.framebuffer,
            background: OPAQUE_BLACK,
            foreground: OPAQUE_BLACK,
        }
    }
}
