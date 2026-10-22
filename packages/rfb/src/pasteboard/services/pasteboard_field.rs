use super::pasteboard_number;

pub(crate) fn pasteboard_field<'a>(rest: &mut &'a [u8]) -> Option<&'a [u8]> {
    let length = usize::try_from(pasteboard_number(rest)?).ok()?;
    let (field, tail) = rest.split_at_checked(length)?;
    *rest = tail;
    Some(field)
}
