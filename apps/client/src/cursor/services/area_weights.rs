pub fn area_weights(source: u16, target: u16) -> Vec<Vec<(usize, f32)>> {
    let ratio = f64::from(source) / f64::from(target);
    (0..target)
        .map(|index| {
            let start = f64::from(index) * ratio;
            let end = start + ratio;
            let first = start.floor() as usize;
            let last = (end.ceil() as usize).min(usize::from(source));
            (first..last)
                .map(|pixel| (pixel, end.min(pixel as f64 + 1.0) - start.max(pixel as f64)))
                .filter(|&(_, overlap)| overlap > 0.0)
                .map(|(pixel, overlap)| (pixel, (overlap / ratio) as f32))
                .collect()
        })
        .collect()
}
