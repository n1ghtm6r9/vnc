use rfb::Rect;

use super::{MvsBits, MvsFullPass, MvsState};
use crate::{
    DecodeError, Framebuffer, MVS_DEFAULT_CLEAR_COLOUR, MVS_FULL_UPDATE_END, MVS_WHITE,
    OPAQUE_BLACK, expect_marker, read_tile_run, tile_grid, tile_rect,
};

impl MvsState {
    pub(super) fn full_update(
        &mut self,
        data: &[u8],
        rect: Rect,
        framebuffer: &mut Framebuffer,
    ) -> Result<(), DecodeError> {
        let [_, threshold1, threshold2, high, middle, low, ..] = *data else {
            return Err(DecodeError::Corrupt(
                "the MVS full update is shorter than its header".to_owned(),
            ));
        };
        let body_offset = (usize::from(high) << 16) | (usize::from(middle) << 8) | usize::from(low);
        if body_offset < 6 || body_offset > data.len() {
            return Err(DecodeError::Corrupt(format!(
                "the MVS full update puts its body at byte {body_offset} of {}",
                data.len()
            )));
        }
        let mut kinds = MvsBits::new(&data[6..body_offset]);
        kinds.read_bit()?;
        let mut pass = MvsFullPass {
            body: MvsBits::new(&data[body_offset..]),
            thresholds: [threshold1, threshold2],
            previous_dct: None,
            previous_tile: (i32::from(rect.x), i32::from(rect.y)),
        };
        self.frame = self.frame.wrapping_add(1);
        self.previous = [[0; 64]; 3];
        self.solid = OPAQUE_BLACK;
        self.set_colour = MVS_WHITE;
        self.clear_colour = MVS_DEFAULT_CLEAR_COLOUR;
        let (tiles_across, tile_count) = tile_grid(rect);
        let mut index = 0;
        while index < tile_count {
            let kind = kinds.read_bits(3)?;
            let run = read_tile_run(&mut kinds)?;
            for _ in 0..run.min(tile_count - index) {
                let tile = tile_rect(rect, tiles_across, index);
                self.full_tile(&mut pass, framebuffer, kind, tile)?;
                index += 1;
            }
        }
        expect_marker(&mut kinds, 8, MVS_FULL_UPDATE_END)?;
        expect_marker(&mut pass.body, 8, MVS_FULL_UPDATE_END)
    }
}
