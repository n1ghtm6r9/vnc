use super::MvsState;

impl MvsState {
    pub(super) fn record_match(
        &mut self,
        screen_index: usize,
        reference: Option<usize>,
        source: (i32, i32),
    ) {
        if let Some(descriptor) = self.descriptors.get_mut(screen_index) {
            descriptor.reference = reference;
            descriptor.match_source = source;
        }
    }
}
