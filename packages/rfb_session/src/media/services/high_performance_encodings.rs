use rfb::{ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MEDIA_STREAM};

use crate::apple_encodings;

pub(crate) fn high_performance_encodings(encodings: &[i32]) -> Vec<i32> {
    let media = [ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MEDIA_STREAM];
    let pixels: Vec<i32> = encodings
        .iter()
        .copied()
        .filter(|encoding| !media.contains(encoding))
        .collect();
    let mut all = vec![ENCODING_APPLE_HIGH_PERFORMANCE];
    all.extend(apple_encodings(&pixels));
    all.push(ENCODING_APPLE_MEDIA_STREAM);
    all
}
