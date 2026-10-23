use crate::names_from_multi_string;

#[test]
fn test_a_double_null_list_splits_into_names() {
    let buffer: Vec<u16> = "uk-UA\0ru-RU\0en-US\0\0".encode_utf16().collect();
    assert_eq!(
        names_from_multi_string(&buffer),
        ["uk-UA", "ru-RU", "en-US"],
        "Windows lists the languages one after another, each ending in a null"
    );
}
