use crate::{ENCODING_CURSOR, Rect, RectHeader};

#[test]
fn test_a_rect_header_round_trips_with_a_pseudo_encoding() {
    let header = RectHeader { rect: Rect::new(4, 5, 32, 32), encoding: ENCODING_CURSOR };
    let mut bytes = Vec::new();
    header.write(&mut bytes).unwrap();
    assert_eq!(bytes.len(), 12, "a rect header is four u16 and one i32");
    assert_eq!(RectHeader::read(&mut &bytes[..]).unwrap(), header, "the negative encoding reads back unchanged");
}
