pub fn nal_unit_starts(stream: &[u8]) -> Vec<(usize, usize)> {
    stream
        .windows(3)
        .enumerate()
        .filter(|(_, window)| *window == [0, 0, 1])
        .map(|(code, _)| {
            let start = if code > 0 && stream[code - 1] == 0 {
                code - 1
            } else {
                code
            };
            (start, code + 3)
        })
        .filter(|&(_, header)| header + 1 < stream.len())
        .collect()
}
