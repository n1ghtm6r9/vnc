use crate::{HORIZONTAL_ENTRY, NEAREST_ENTRY, RESAMPLE_WGSL, VERTEX_ENTRY, VERTICAL_ENTRY};
use wgpu::naga::front::wgsl::parse_str;
use wgpu::naga::valid::{Capabilities, ValidationFlags, Validator};

#[test]
fn test_the_resample_shader_declares_every_entry_point() {
    let module = parse_str(RESAMPLE_WGSL).expect("the resample shader must parse");
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .expect("the resample shader must validate");
    for entry in [
        VERTEX_ENTRY,
        HORIZONTAL_ENTRY,
        VERTICAL_ENTRY,
        NEAREST_ENTRY,
    ] {
        assert!(
            module.entry_points.iter().any(|point| point.name == entry),
            "the resample shader must declare the entry point {entry}"
        );
    }
}
