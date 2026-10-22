use wgpu::TextureFormat;

use crate::choose_surface_format;

#[test]
fn test_a_linear_8_bit_surface_format_is_preferred() {
    let format = choose_surface_format(&[TextureFormat::Bgra8UnormSrgb, TextureFormat::Rgba16Float, TextureFormat::Bgra8Unorm]);
    assert_eq!(format, Some(TextureFormat::Bgra8Unorm), "egui blends in gamma space, so the swapchain is linear and the image gets an sRGB view");
    let fallback = choose_surface_format(&[TextureFormat::Rgba8UnormSrgb, TextureFormat::Rgb10a2Unorm]);
    assert_eq!(fallback, Some(TextureFormat::Rgb10a2Unorm), "any linear format beats an sRGB one");
    assert_eq!(choose_surface_format(&[]), None, "a surface without formats gives nothing");
}
