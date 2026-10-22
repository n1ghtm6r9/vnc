use super::HevcDecoder;

impl Drop for HevcDecoder {
    fn drop(&mut self) {
        let cuda = unsafe { (*self.state).cuda };
        let destroyed = unsafe { (cuda.destroy_video_parser)(self.parser) };
        if let Err(error) = cuda.check("cuvidDestroyVideoParser", destroyed) {
            log::warn!("[DECODE] {error}");
        }
        drop(unsafe { Box::from_raw(self.state) });
    }
}
