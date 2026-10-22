use std::{
    net::IpAddr,
    sync::{Arc, atomic::AtomicBool},
};

use ard_media::virtual_display_for;
use hp_media::{MAC_MEDIA_PORTS, MediaOptions};
use log::{info, warn};
use rfb::apple_offers_high_performance;

use super::SessionThread;
use crate::{MediaForwarder, SessionMedia};

impl SessionThread {
    pub(super) fn start_high_performance(
        &mut self,
        server: IpAddr,
        capabilities: Option<[u8; 16]>,
    ) -> Option<SessionMedia> {
        let wanted = self.options.high_performance?;
        if !capabilities.is_some_and(|capabilities| apple_offers_high_performance(&capabilities)) {
            info!("[SESSION] the Mac does not offer High Performance, the session stays in standard mode");
            return None;
        }
        let streaming = Arc::new(AtomicBool::new(false));
        let forwarder = MediaForwarder {
            events: self.events.clone(),
            outgoing: self.outgoing.clone(),
            streaming: Arc::clone(&streaming),
        };
        let options = MediaOptions {
            server,
            ports: MAC_MEDIA_PORTS,
            play_sound: wanted.play_sound,
        };
        match (self.start_media)(options, Box::new(move |event| forwarder.forward(event))) {
            Ok(peer) => {
                let display = virtual_display_for(wanted.display_width, wanted.display_height, wanted.display_scale);
                if let Some(mode) = display.modes.first() {
                    info!(
                        "[SESSION] High Performance starts with a virtual display of {}x{} points, {}x{} pixels",
                        mode.width, mode.height, mode.pixel_width, mode.pixel_height
                    );
                }
                Some(SessionMedia {
                    peer: Arc::from(peer),
                    streaming,
                    display,
                })
            }
            Err(error) => {
                warn!("[SESSION] High Performance cannot start, the session stays in standard mode: {error}");
                None
            }
        }
    }
}
