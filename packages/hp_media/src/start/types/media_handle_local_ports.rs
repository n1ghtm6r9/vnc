use super::MediaHandle;

impl MediaHandle {
    pub fn local_ports(&self) -> [u16; 2] {
        self.local_ports
    }
}
