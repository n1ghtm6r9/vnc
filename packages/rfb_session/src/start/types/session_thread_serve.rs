use std::sync::{Arc, mpsc::Receiver};

use log::info;
use rfb::Rect;

use super::SessionThread;
use crate::{
    Handshake, Outgoing, SessionError, SessionEvent, SessionReader, SessionStream, SessionWriter,
    connect_tcp, perform_handshake, send_apple_setup, send_setup, spawn_writer,
};

impl SessionThread {
    pub(super) fn serve(&mut self, queue: Receiver<Outgoing>) -> Result<(), SessionError> {
        let options = &self.options;
        let socket = connect_tcp(&options.host, options.port, options.connect_timeout)
            .map_err(SessionError::Connect)?;
        let server = socket.peer_addr().map_err(SessionError::Connect)?.ip();
        let mut stream =
            SessionStream::new(socket, Arc::clone(&self.control), options.connect_timeout)?;
        let Handshake {
            init,
            apple_key,
            capabilities,
        } = perform_handshake(&mut stream, options)?;
        let screen = Rect::new(0, 0, init.width, init.height);
        let media = self.start_high_performance(server, capabilities);
        let display = media.as_ref().map(|media| &media.display);
        match capabilities {
            Some(_) => send_apple_setup(&mut stream, &self.options.encodings, screen, display)?,
            None => send_setup(&mut stream, &self.options.encodings, screen)?,
        }
        stream.lift_deadline();
        let apple_remote_desktop = apple_key.is_some();
        let streaming = media.as_ref().map(|media| Arc::clone(&media.streaming));
        let display_control = self
            .options
            .high_performance
            .zip(media.as_ref())
            .map(|(wanted, media)| media.display_control(wanted.display_scale));
        spawn_writer(
            SessionWriter::new(stream.try_clone_socket()?, queue, screen, apple_key)
                .with_media(streaming)
                .with_display(display_control),
        )?;
        info!(
            "[SESSION] connected to {:?}, {}x{}, Apple Remote Desktop mode: {apple_remote_desktop}",
            init.name, init.width, init.height
        );
        self.events.emit(SessionEvent::Connected {
            width: init.width,
            height: init.height,
            name: init.name,
            apple_remote_desktop,
        });
        let mut reader =
            SessionReader::new(stream, screen, self.outgoing.clone(), apple_remote_desktop)
                .with_media(media);
        let events = self.events.clone();
        Err(reader.run(&mut |event| events.emit(event)))
    }
}
