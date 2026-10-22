use super::MediaPhase;

impl MediaPhase {
    pub(crate) fn into_streaming(self) -> MediaPhase {
        match self {
            MediaPhase::Waiting { stream, .. } => MediaPhase::Streaming {
                stream,
                reoffer: None,
            },
            other => other,
        }
    }
}
