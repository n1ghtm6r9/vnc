use crate::test_support::{
    ImageOptions, TEST_TARGET_FORMAT, TargetOptions, TestGpu, assert_bytes_close, pixel_at,
    random_image, render_image,
};
use crate::{ScaleFilter, Viewport};

#[test]
fn test_a_viewport_hanging_off_the_target_draws_only_its_visible_part() {
    let Some(gpu) = TestGpu::new() else {
        return;
    };
    let source = random_image(4, 4, 31);
    let background = wgpu::Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    let render = |viewport| {
        render_image(
            &gpu,
            ImageOptions {
                source_width: 4,
                source_height: 4,
                source: &source,
                target: TargetOptions {
                    format: TEST_TARGET_FORMAT,
                    width: 4,
                    height: 4,
                    viewport,
                    filter: ScaleFilter::Nearest,
                    background,
                },
            },
        )
    };

    let hanging = render(Viewport {
        x: -2,
        y: -1,
        width: 4,
        height: 4,
    });
    assert_bytes_close(
        &pixel_at(&hanging, 4, 0, 0),
        &pixel_at(&source, 4, 2, 1),
        1,
        "the target pixel 0,0 shows the source pixel 2,1",
    );
    assert_bytes_close(
        &pixel_at(&hanging, 4, 1, 2),
        &pixel_at(&source, 4, 3, 3),
        1,
        "the target pixel 1,2 shows the source pixel 3,3",
    );
    for y in 0..4 {
        for x in 0..4 {
            let visible = x < 2 && y < 3;
            let expected = if visible {
                pixel_at(&source, 4, x + 2, y + 1)
            } else {
                [0, 255, 0, 255]
            };
            assert_bytes_close(
                &pixel_at(&hanging, 4, x, y),
                &expected,
                1,
                &format!("the target pixel {x},{y} (inside the visible part: {visible})"),
            );
        }
    }

    let beside = render(Viewport {
        x: 10,
        y: 0,
        width: 4,
        height: 4,
    });
    for y in 0..4 {
        for x in 0..4 {
            assert_bytes_close(
                &pixel_at(&beside, 4, x, y),
                &[0, 255, 0, 255],
                1,
                &format!("the target pixel {x},{y} with the viewport entirely beside the target"),
            );
        }
    }
}
