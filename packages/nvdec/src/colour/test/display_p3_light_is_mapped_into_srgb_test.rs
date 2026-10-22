use crate::ColourPrimaries;

#[test]
fn test_display_p3_light_is_mapped_into_srgb() {
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let p3 = ColourPrimaries::DisplayP3.to_srgb();

    assert_eq!(
        ColourPrimaries::Bt709.to_srgb(),
        identity,
        "BT.709 light already is sRGB light"
    );
    for row in p3 {
        assert!(
            (row.iter().sum::<f32>() - 1.0).abs() < 1e-5,
            "white stays white, but the row {row:?} does not add up to one"
        );
    }
    let red = p3.map(|row| row[0]);
    assert!(
        red[0] > 1.2 && red[1] < 0.0 && red[2] < 0.0,
        "pure P3 red is redder than sRGB can show, got {red:?}"
    );
}
