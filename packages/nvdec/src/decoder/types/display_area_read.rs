use super::DisplayArea;
use crate::CuvidVideoFormat;

impl DisplayArea {
    pub(crate) fn read(video: &CuvidVideoFormat) -> Self {
        let left = u32::try_from(video.display_left).unwrap_or(0);
        let top = u32::try_from(video.display_top).unwrap_or(0);
        let right = u32::try_from(video.display_right)
            .unwrap_or(0)
            .min(video.coded_width);
        let bottom = u32::try_from(video.display_bottom)
            .unwrap_or(0)
            .min(video.coded_height);
        if right <= left || bottom <= top {
            return Self {
                left: 0,
                top: 0,
                width: video.coded_width,
                height: video.coded_height,
            };
        }
        Self {
            left,
            top,
            width: right - left,
            height: bottom - top,
        }
    }
}
