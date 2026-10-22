use crate::{BIG_ICON_SIDE, SMALL_ICON_SIDE, icon_side};

#[test]
fn test_the_icon_file_has_an_image_for_every_common_scale() {
    let file = include_bytes!("../../../assets/vnc.ico");
    let count = usize::from(u16::from_le_bytes([file[4], file[5]]));
    let sides: Vec<u32> = (0..count)
        .map(|index| match file[6 + index * 16] {
            0 => 256,
            side => u32::from(side),
        })
        .collect();
    for scale in [1.0, 1.25, 1.5, 2.0] {
        for side in [SMALL_ICON_SIDE, BIG_ICON_SIDE] {
            let wanted = icon_side(side, scale);
            assert!(sides.contains(&wanted), "the icon file has a {wanted} pixel image for the {scale} scale");
        }
    }
    assert!(sides.contains(&256), "the icon file has a 256 pixel image for large icons in Explorer");
}
