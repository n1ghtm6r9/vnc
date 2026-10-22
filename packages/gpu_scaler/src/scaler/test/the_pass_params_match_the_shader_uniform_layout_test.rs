use crate::PassParams;

#[test]
fn test_the_pass_params_match_the_shader_uniform_layout() {
    assert_eq!(
        size_of::<PassParams>(),
        32,
        "the uniform struct of the shader takes 32 bytes"
    );
    let params = PassParams::new([-3, 4], [5, 6], [7, 8], true);
    let words: [i32; 8] = bytemuck::cast(params);
    assert_eq!(
        words,
        [-3, 4, 5, 6, 7, 8, 1, 0],
        "the fields follow the order origin, input size, output size, srgb flag, padding"
    );
    let plain = PassParams::new([0, 0], [1, 1], [1, 1], false);
    let words: [i32; 8] = bytemuck::cast(plain);
    assert_eq!(
        words[6], 0,
        "a linear target must not ask for the srgb transfer function"
    );
}
