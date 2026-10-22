use super::MediaPhase;
use crate::MediaStream;

impl MediaPhase {
    pub(crate) fn stream_mut(&mut self) -> Option<&mut MediaStream> {
        match self {
            MediaPhase::Waiting { stream, .. } | MediaPhase::Streaming { stream, .. } => {
                Some(stream)
            }
            MediaPhase::AwaitingMessage1 { .. }
            | MediaPhase::Offering(_)
            | MediaPhase::Finished => None,
        }
    }
}
