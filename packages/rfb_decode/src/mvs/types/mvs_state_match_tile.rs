use rfb::Rect;

use super::MvsState;
use crate::{Framebuffer, copy_tile};

impl MvsState {
    pub(super) fn match_tile(
        &self,
        framebuffer: &mut Framebuffer,
        screen_index: usize,
        tile: Rect,
    ) -> bool {
        let Some(descriptor) = self.descriptors.get(screen_index) else {
            return false;
        };
        let reference = descriptor
            .reference
            .and_then(|reference| self.descriptors.get(reference));
        let Some(reference) = reference else {
            return false;
        };
        if descriptor.frame != reference.frame {
            return false;
        }
        copy_tile(framebuffer, descriptor.match_source, tile);
        true
    }
}
