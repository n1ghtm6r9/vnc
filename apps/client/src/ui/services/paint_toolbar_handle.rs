use egui::{Color32, Context, Id, LayerId, Order, Rect, Vec2, pos2};

pub fn paint_toolbar_handle(ctx: &Context) {
    let centre = pos2(ctx.content_rect().center().x, 3.0);
    let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::unique("toolbar_handle")));
    painter.rect_filled(
        Rect::from_center_size(centre, Vec2::new(56.0, 4.0)),
        2.0,
        Color32::from_white_alpha(96),
    );
}
