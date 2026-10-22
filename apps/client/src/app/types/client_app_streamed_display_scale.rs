use super::ClientApp;

impl ClientApp {
    pub(super) fn streamed_display_scale(&self) -> Option<u32> {
        self.media_picture
            .and(self.last_options.as_ref().and_then(|options| options.high_performance))
            .map(|high_performance| high_performance.display_scale)
    }
}
