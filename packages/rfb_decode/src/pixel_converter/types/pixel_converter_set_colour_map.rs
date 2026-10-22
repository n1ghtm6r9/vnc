use super::PixelConverter;
use crate::OPAQUE_BLACK;

impl PixelConverter {
    pub fn set_colour_map(&mut self, first: u16, colours: &[[u16; 3]]) {
        let start = usize::from(first);
        let end = start + colours.len();
        if self.colour_map.len() < end {
            self.colour_map.resize(end, OPAQUE_BLACK);
        }
        for (slot, colour) in self.colour_map[start..end].iter_mut().zip(colours) {
            let [red, green, blue] = colour.map(|intensity| intensity.to_be_bytes()[0]);
            *slot = u32::from_le_bytes([red, green, blue, 0xFF]);
        }
    }
}
