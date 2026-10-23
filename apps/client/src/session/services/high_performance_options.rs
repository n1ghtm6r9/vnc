use rfb_session::HighPerformanceOptions;
use winit::dpi::PhysicalSize;

use crate::{RETINA_DISPLAY_SCALE, STANDARD_DISPLAY_SCALE, SoundPlace};

pub fn high_performance_options(display: PhysicalSize<u32>, retina: bool, sound: SoundPlace) -> HighPerformanceOptions {
    HighPerformanceOptions {
        display_width: display.width,
        display_height: display.height,
        display_scale: if retina { RETINA_DISPLAY_SCALE } else { STANDARD_DISPLAY_SCALE },
        play_sound: sound == SoundPlace::Here,
    }
}
