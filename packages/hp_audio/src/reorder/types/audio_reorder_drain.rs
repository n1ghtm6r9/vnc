use super::{AudioReorder, OrderedFrame};
use crate::REORDER_DEPTH;

impl AudioReorder {
    pub(super) fn drain(&mut self, mut next: i64) -> Vec<OrderedFrame> {
        let mut frames = Vec::new();
        loop {
            if let Some(payload) = self.waiting.remove(&next) {
                frames.push(OrderedFrame { lost: 0, payload });
                next += 1;
            } else if self.waiting.len() > REORDER_DEPTH
                && let Some((first, payload)) = self.waiting.pop_first()
            {
                let lost = usize::try_from(first - next).unwrap_or(usize::MAX);
                frames.push(OrderedFrame { lost, payload });
                next = first + 1;
            } else {
                break;
            }
        }
        self.next = Some(next);
        frames
    }
}
