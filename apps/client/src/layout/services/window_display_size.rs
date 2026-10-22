use winit::dpi::PhysicalSize;

pub fn window_display_size(window: PhysicalSize<u32>) -> Option<PhysicalSize<u32>> {
    let even = |side: u32| side / 2 * 2;
    let size = PhysicalSize::new(even(window.width), even(window.height));
    (size.width > 0 && size.height > 0).then_some(size)
}
