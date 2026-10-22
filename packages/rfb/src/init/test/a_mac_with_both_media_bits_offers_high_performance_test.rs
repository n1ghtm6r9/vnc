use crate::apple_offers_high_performance;

#[test]
fn test_a_mac_with_both_media_bits_offers_high_performance() {
    let capabilities = |fourth: u8| {
        let mut bitmap = [0u8; 16];
        bitmap[..5].copy_from_slice(&[0xbf, 0xf6, 0xe7, fourth, 0xec]);
        bitmap
    };

    assert!(apple_offers_high_performance(&capabilities(0x2f)), "an Apple silicon Mac sets both bits of byte 3");
    assert!(!apple_offers_high_performance(&capabilities(0x23)), "an Intel Mac lacks one of them and is not asked");
    assert!(!apple_offers_high_performance(&capabilities(0x08)), "one bit alone is not enough");
    assert!(!apple_offers_high_performance(&[0; 16]), "a server without a bitmap is not asked");
}
