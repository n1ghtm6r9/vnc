pub(crate) trait MediaPeer: Send + Sync {
    fn message1(&self, payload: Vec<u8>);
    fn answer(&self, payload: Vec<u8>);
    fn resize(&self, display: Vec<u8>);
    fn play_sound(&self, play: bool);
}
