use std::time::Instant;

use super::MediaStream;

impl MediaStream {
    pub(crate) fn diagnosis(&self, now: Instant) -> String {
        let ago = |at: Instant| now.saturating_duration_since(at).as_millis();
        let video = self.last_video_packet_at.map_or_else(
            || "no video packet yet".to_owned(),
            |at| format!("last video packet {} ms ago", ago(at)),
        );
        let held = self.assembler.held_since().map_or_else(
            || "nothing held".to_owned(),
            |at| format!("a picture held for {} ms", ago(at)),
        );
        let stats = self.assembler.stats();
        format!(
            "last picture {} ms ago, {video}, last media {} ms ago, {held}; packets {}, pictures {}, ignored {}, malformed {}, duplicates {}, discarded {}, skipped {}",
            ago(self.last_picture_at),
            ago(self.last_media_at),
            stats.packets,
            stats.pictures,
            stats.ignored,
            stats.malformed,
            stats.duplicates,
            stats.discarded,
            stats.skipped,
        )
    }
}
