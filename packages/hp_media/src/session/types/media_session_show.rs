use std::time::Instant;

use ard_media::{VideoPicture, picture_ack_packet};

use super::MediaSession;
use crate::{FrameDecoder, MediaChannel, MediaEvent, MediaFailure, StripeImage, stripe_image};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn show(&mut self, picture: VideoPicture, now: Instant) {
        let Some(stream) = self.phase.stream_mut() else {
            return;
        };
        let ack = picture_ack_packet(stream.secrets.video_ssrc, picture.don as u16);
        self.effects
            .datagrams
            .extend(stream.protect(MediaChannel::Video, &ack));
        self.pictures += 1;
        let video = stream.answer.video;
        let frames = match self.decoder.decode(&picture.data) {
            Ok(frames) => frames,
            Err(error) => {
                self.fail(MediaFailure::Decoder(error.to_string()));
                return;
            }
        };
        let images: Vec<StripeImage> = frames
            .into_iter()
            .filter_map(|frame| stripe_image(frame, picture.stripe, video.width, video.height))
            .collect();
        if images.is_empty() {
            return;
        }
        stream.last_picture_at = now;
        self.mark_shown();
        self.meter.pictures += images.len() as u64;
        self.effects
            .events
            .extend(images.into_iter().map(MediaEvent::Stripe));
    }
}
