pub fn nal_units(stream: &[u8]) -> Vec<&[u8]> {
    let starts: Vec<(usize, usize)> = stream
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
        .collect();
    starts
        .iter()
        .enumerate()
        .map(|(index, &(_, header))| {
            let end = starts
                .get(index + 1)
                .map_or(stream.len(), |&(start, _)| start);
            &stream[header..end]
        })
        .collect()
}
