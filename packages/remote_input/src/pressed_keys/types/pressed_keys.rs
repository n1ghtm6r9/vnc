use winit::keyboard::PhysicalKey;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PressedKeys {
    pub(super) keys: Vec<(PhysicalKey, u32)>,
}
