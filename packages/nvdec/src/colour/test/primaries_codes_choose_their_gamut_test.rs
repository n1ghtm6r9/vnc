use crate::ColourPrimaries;

#[test]
fn test_primaries_codes_choose_their_gamut() {
    let cases = [
        (1, ColourPrimaries::Bt709),
        (2, ColourPrimaries::Bt709),
        (12, ColourPrimaries::DisplayP3),
        (99, ColourPrimaries::Bt709),
    ];

    for (code, expected) in cases {
        assert_eq!(
            ColourPrimaries::from_code(code),
            expected,
            "primaries code {code} picks its gamut, and other codes are shown as sRGB"
        );
    }
}
