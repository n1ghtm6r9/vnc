use std::{fs::File, io::BufWriter, path::Path};

use png::{BitDepth, ColorType, Encoder};

use crate::{Snapshot, SnapshotError};

impl Snapshot {
    pub(crate) fn save_png(&self, path: &Path) -> Result<(), SnapshotError> {
        let file = File::create(path)?;
        let mut encoder = Encoder::new(
            BufWriter::new(file),
            u32::from(self.width),
            u32::from(self.height),
        );
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        let bytes: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|pixel| pixel.to_le_bytes())
            .collect();
        writer.write_image_data(&bytes)?;
        writer.finish()?;
        Ok(())
    }
}
