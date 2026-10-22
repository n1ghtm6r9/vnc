use crate::ColourMatrix;

#[test]
fn test_matrix_codes_choose_their_standard() {
    let cases = [
        (0, ColourMatrix::Identity),
        (1, ColourMatrix::Bt709),
        (2, ColourMatrix::Bt709),
        (4, ColourMatrix::Bt601),
        (5, ColourMatrix::Bt601),
        (6, ColourMatrix::Bt601),
        (9, ColourMatrix::Bt2020),
        (10, ColourMatrix::Bt2020),
        (99, ColourMatrix::Bt709),
    ];

    for (code, expected) in cases {
        assert_eq!(
            ColourMatrix::from_code(code),
            expected,
            "matrix code {code} picks its standard, and unknown codes fall back to BT.709"
        );
    }
}
