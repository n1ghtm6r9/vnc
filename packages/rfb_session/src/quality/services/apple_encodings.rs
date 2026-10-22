use rfb::APPLE_PSEUDO_ENCODINGS;

pub(crate) fn apple_encodings(encodings: &[i32]) -> Vec<i32> {
    let mut all = encodings.to_vec();
    for encoding in APPLE_PSEUDO_ENCODINGS {
        if !all.contains(&encoding) {
            all.push(encoding);
        }
    }
    all
}
