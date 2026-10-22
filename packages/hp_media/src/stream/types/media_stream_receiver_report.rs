use std::time::Instant;

use ard_media::ReceiverReport;

use super::MediaStream;
use crate::MediaChannel;

impl MediaStream {
    pub(crate) fn receiver_report(
        &self,
        channel: MediaChannel,
        now: Instant,
    ) -> Option<ReceiverReport> {
        let index = channel.index();
        let highest = self.highest_seq[index]?;
        let (ssrc, source_ssrc) = match channel {
            MediaChannel::Audio => (self.secrets.audio_ssrc, self.answer.audio_ssrc),
            MediaChannel::Video => (self.secrets.video_ssrc, self.answer.video.ssrc),
        };
        let (last_sender_report, delay_since_sender_report) =
            self.sender_reports[index].map_or((0, 0), |report| {
                let delay = now.saturating_duration_since(report.at).as_secs_f64() * 65536.0;
                (report.middle, delay.round() as u32)
            });
        Some(ReceiverReport {
            ssrc,
            source_ssrc,
            highest_seq: highest as u32,
            last_sender_report,
            delay_since_sender_report,
            ..ReceiverReport::default()
        })
    }
}
