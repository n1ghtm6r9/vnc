use rfb_session::Quality;

use crate::ImageQuality;

pub fn session_quality(quality: ImageQuality) -> Quality {
    match quality {
        ImageQuality::Lossless => Quality::Lossless,
        ImageQuality::Balanced => Quality::Balanced,
        ImageQuality::Fast => Quality::Fast,
    }
}
