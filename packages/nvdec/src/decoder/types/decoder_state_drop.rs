use super::DecoderState;

impl Drop for DecoderState {
    fn drop(&mut self) {
        if let Err(error) = self.destroy_decoder() {
            log::warn!("[DECODE] {error}");
        }
        let released = unsafe { (self.cuda.device_primary_ctx_release)(self.primary.device) };
        if let Err(error) = self.cuda.check("cuDevicePrimaryCtxRelease", released) {
            log::warn!("[DECODE] {error}");
        }
    }
}
