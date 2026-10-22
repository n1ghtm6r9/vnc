use super::display_record;
use crate::{MSG_VIRTUAL_DISPLAY, VirtualDisplay};

pub fn virtual_display_message(displays: &[VirtualDisplay]) -> Vec<u8> {
    let records: Vec<Vec<u8>> = displays.iter().map(display_record).collect();
    let total = 12 + records.iter().map(Vec::len).sum::<usize>();
    let mut message = Vec::with_capacity(total);
    message.extend_from_slice(&[MSG_VIRTUAL_DISPLAY, 0]);
    message.extend_from_slice(&((total - 4) as u16).to_be_bytes());
    message.extend_from_slice(&1u16.to_be_bytes());
    message.extend_from_slice(&(displays.len() as u16).to_be_bytes());
    message.extend_from_slice(&[0; 4]);
    for record in records {
        message.extend_from_slice(&record);
    }
    message
}
