use rfb::RectHeader;
use rfb_decode::{DecodeTarget, RectOutcome, decode_rect};

use super::SessionReader;
use crate::{SessionError, SessionEvent, collect_patches};

impl SessionReader {
    pub(super) fn read_update(
        &mut self,
        rect_count: u16,
        on_event: &mut dyn FnMut(SessionEvent),
    ) -> Result<(), SessionError> {
        if !self.apple_remote_desktop {
            self.request_update(true);
        }
        let mut changed = Vec::new();
        let mut resized = false;
        for _ in 0..rect_count {
            let header = RectHeader::read(&mut self.input)?;
            let target = DecodeTarget {
                converter: &self.converter,
                state: &mut self.decoder,
                framebuffer: &mut self.framebuffer,
            };
            let outcome = decode_rect(&mut self.input, &header, target)?;
            if let RectOutcome::AppleDisplay {
                geometry: Some(geometry),
                ..
            } = outcome
            {
                on_event(SessionEvent::DisplayGeometry(geometry));
            }
            match outcome {
                RectOutcome::Pixels(rect) => {
                    self.last_encoding = header.encoding;
                    changed.push(rect);
                }
                RectOutcome::Cursor(shape) => on_event(SessionEvent::CursorChanged(shape)),
                RectOutcome::SystemCursor(cursor) => {
                    on_event(SessionEvent::SystemCursorChanged(cursor));
                }
                RectOutcome::DesktopSize { width, height }
                | RectOutcome::AppleDisplay { width, height, .. } => {
                    changed.clear();
                    resized = true;
                    on_event(SessionEvent::Resized { width, height });
                }
                RectOutcome::AppleMedia { encoding, payload } => self.pass_media(encoding, payload),
                RectOutcome::AppleInputSource(source) => self.report_input_source(source, on_event),
                RectOutcome::Ignored => {}
                RectOutcome::LastRect => break,
            }
        }
        let streaming = self.media_streaming();
        if !streaming {
            let patches = collect_patches(&self.framebuffer, &changed);
            if !patches.is_empty() {
                on_event(SessionEvent::FrameUpdated { patches });
            }
        }
        if resized {
            self.refetch_screen();
        } else if streaming {
            self.renew_pushed_updates();
        }
        if !streaming {
            self.report_stats(on_event);
        }
        Ok(())
    }
}
