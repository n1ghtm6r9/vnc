use crate::MediaEvent;

pub fn offer_count(events: &[MediaEvent]) -> usize {
    events
        .iter()
        .filter(|event| matches!(event, MediaEvent::Offer(_)))
        .count()
}
