use super::{MediaState, MediaStats, StripeImage};

#[derive(Debug, Clone, PartialEq)]
pub enum MediaEvent {
    Display(Vec<u8>),
    Offer(Vec<u8>),
    Stripe(StripeImage),
    State(MediaState),
    Stats(MediaStats),
}
