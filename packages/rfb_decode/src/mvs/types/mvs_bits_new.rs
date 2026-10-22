use super::MvsBits;

impl<'a> MvsBits<'a> {
    pub(crate) fn new(data: &'a [u8]) -> MvsBits<'a> {
        MvsBits { data, position: 0 }
    }
}
