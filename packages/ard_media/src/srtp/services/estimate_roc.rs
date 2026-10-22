pub fn estimate_roc(highest: Option<u64>, seq: u16) -> u32 {
    let Some(highest) = highest else {
        return 0;
    };
    let roc = (highest >> 16) as u32;
    let last = i64::from(highest as u16);
    let seq = i64::from(seq);
    if last < 0x8000 {
        if seq - last > 0x8000 && roc > 0 {
            roc - 1
        } else {
            roc
        }
    } else if last - 0x8000 > seq {
        roc.wrapping_add(1)
    } else {
        roc
    }
}
