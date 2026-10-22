use crate::extend_seq;

#[test]
fn test_a_wire_number_extends_to_the_nearest_value() {
    assert_eq!(
        extend_seq(65530, 3),
        65539,
        "a small number after a high one has wrapped forward"
    );
    assert_eq!(
        extend_seq(65539, 65534),
        65534,
        "a high number after the wrap is a late one from before it"
    );
    assert_eq!(extend_seq(10, 12), 12, "a nearby number keeps its value");
    assert_eq!(
        extend_seq(3, 65535),
        -1,
        "a number just before the start lies below zero"
    );
}
