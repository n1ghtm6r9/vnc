use crate::{CONVERT_ENTRY, VERTEX_ENTRY, YUV_WGSL};
use wgpu::naga::front::wgsl::parse_str;
use wgpu::naga::valid::{Capabilities, ValidationFlags, Validator};

#[test]
fn test_the_yuv_shader_declares_its_entry_points() {
    let module = parse_str(YUV_WGSL).expect("the yuv shader must parse");
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .expect("the yuv shader must validate");
    for entry in [VERTEX_ENTRY, CONVERT_ENTRY] {
        assert!(
            module.entry_points.iter().any(|point| point.name == entry),
            "the yuv shader must declare the entry point {entry}"
        );
    }
}
