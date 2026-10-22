use crate::split_key_block;

#[test]
fn test_a_key_block_splits_into_the_two_directions() {
    let block: [u8; 92] = std::array::from_fn(|index| index as u8);

    let keys = split_key_block(&block);

    assert_eq!(
        keys.to_server.key[0], 0,
        "the key to the server comes first"
    );
    assert_eq!(keys.to_server.salt[0], 32, "its salt follows at byte 32");
    assert_eq!(
        keys.from_server.key[0], 46,
        "the key from the server starts at byte 46"
    );
    assert_eq!(keys.from_server.salt[0], 78, "its salt starts at byte 78");
    assert_eq!(
        keys.from_server.salt[13], 91,
        "the salt runs to the end of the block"
    );
}
