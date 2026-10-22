use ard_media::SrtpContext;

pub struct TestServer {
    pub to_client: [SrtpContext; 2],
    pub from_client: [SrtpContext; 2],
    pub seq: u16,
    pub audio_seq: u16,
    pub payload_type: u8,
}
