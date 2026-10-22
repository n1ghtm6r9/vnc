use rfb_session::Patch;

use crate::Snapshot;

impl Snapshot {
    pub(crate) fn draw_patch(&mut self, patch: &Patch) {
        let rect = patch.rect;
        let row_length = usize::from(rect.width);
        if row_length == 0 {
            return;
        }
        for (row, source) in patch.pixels.chunks_exact(row_length).enumerate() {
            let start = (usize::from(rect.y) + row) * usize::from(self.width) + usize::from(rect.x);
            if let Some(target) = self.pixels.get_mut(start..start + row_length) {
                target.copy_from_slice(source);
            }
        }
    }
}
