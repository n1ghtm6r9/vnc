use super::nal_units;

pub fn access_units(stream: &[u8]) -> Vec<Vec<&[u8]>> {
    let mut units: Vec<Vec<&[u8]>> = Vec::new();
    let mut has_picture = false;
    for nal in nal_units(stream) {
        let kind = (nal[0] >> 1) & 0x3f;
        let first_slice = nal.get(2).is_some_and(|byte| byte & 0x80 != 0);
        let opens_unit =
            matches!(kind, 32..=35 | 39 | 41..=44 | 48..=55) || (kind < 32 && first_slice);
        if units.is_empty() || (has_picture && opens_unit) {
            units.push(Vec::new());
            has_picture = false;
        }
        has_picture |= kind < 32;
        units.last_mut().expect("an access unit is open").push(nal);
    }
    units
}
