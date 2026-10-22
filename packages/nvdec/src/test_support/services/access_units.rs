use super::nal_unit_starts;

pub fn access_units(stream: &[u8]) -> Vec<&[u8]> {
    let mut units = Vec::new();
    let mut unit_start = 0;
    let mut has_picture = false;
    for (start, header) in nal_unit_starts(stream) {
        let kind = (stream[header] >> 1) & 0x3f;
        let first_slice = stream.get(header + 2).is_some_and(|byte| byte & 0x80 != 0);
        let opens_unit =
            matches!(kind, 32..=35 | 39 | 41..=44 | 48..=55) || (kind < 32 && first_slice);
        if has_picture && opens_unit {
            units.push(&stream[unit_start..start]);
            unit_start = start;
            has_picture = false;
        }
        has_picture |= kind < 32;
    }
    units.push(&stream[unit_start..]);
    units
}
