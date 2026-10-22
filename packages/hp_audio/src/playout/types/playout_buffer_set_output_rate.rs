use super::PlayoutBuffer;
use crate::{SincKernel, output_cutoff};

impl PlayoutBuffer {
    pub(crate) fn set_output_rate(&mut self, output_rate: u32) {
        if output_rate != self.output_rate {
            self.output_rate = output_rate;
            self.kernel = SincKernel::new(output_cutoff(output_rate));
        }
    }
}
