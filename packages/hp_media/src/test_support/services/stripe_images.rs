use crate::{MediaEvent, StripeImage};

pub fn stripe_images(events: &[MediaEvent]) -> Vec<StripeImage> {
    events
        .iter()
        .filter_map(|event| match event {
            MediaEvent::Stripe(image) => Some(image.clone()),
            _ => None,
        })
        .collect()
}
