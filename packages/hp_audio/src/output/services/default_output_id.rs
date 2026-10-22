use cpal::DeviceId;
use cpal::traits::{DeviceTrait, HostTrait};

pub(crate) fn default_output_id() -> Option<DeviceId> {
    cpal::default_host().default_output_device()?.id().ok()
}
