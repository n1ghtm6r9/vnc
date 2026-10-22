pub(crate) fn pasteboard_number(rest: &mut &[u8]) -> Option<u32> {
    let (number, tail) = rest.split_first_chunk::<4>()?;
    *rest = tail;
    Some(u32::from_be_bytes(*number))
}
