use std::time::Instant;

use ard_media::PAYLOAD_TYPE_HEVC;
use hp_audio::SoundPacket;

use super::{Datagram, MediaPhase, MediaSession};
use crate::{FrameDecoder, MediaChannel, MediaFailure, Received};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn handle_datagram(
        &mut self,
        channel: MediaChannel,
        datagram: &mut [u8],
        now: Instant,
    ) {
        match &mut self.phase {
            MediaPhase::Offering(offer) => {
                offer.keep(Datagram {
                    channel,
                    bytes: datagram.to_vec(),
                });
                return;
            }
            MediaPhase::Waiting { .. } | MediaPhase::Streaming { .. } => {}
            MediaPhase::AwaitingMessage1 { .. } | MediaPhase::Finished => return,
        }
        self.meter.bytes += datagram.len() as u64;
        let Some(stream) = self.phase.stream_mut() else {
            return;
        };
        let packet = match stream.receive(channel, datagram, now) {
            Received::Video(packet) => packet,
            Received::Rejected => {
                if let MediaPhase::Streaming {
                    reoffer: Some(offer),
                    ..
                } = &mut self.phase
                {
                    offer.keep(Datagram {
                        channel,
                        bytes: datagram.to_vec(),
                    });
                }
                return;
            }
            Received::Audio(packet) => {
                self.effects.sounds.push(SoundPacket {
                    ssrc: packet.ssrc,
                    seq: packet.seq,
                    payload: packet.payload.to_vec(),
                });
                return;
            }
            Received::Control => return,
        };
        if packet.payload_type != PAYLOAD_TYPE_HEVC {
            self.fail(MediaFailure::UnsupportedVideo(packet.payload_type));
            return;
        }
        self.video_packets = self.video_packets.wrapping_add(1);
        let Some(stream) = self.phase.stream_mut() else {
            return;
        };
        stream.assembler.push(&packet, now, &mut self.assembled);
        self.settle(now);
    }
}
