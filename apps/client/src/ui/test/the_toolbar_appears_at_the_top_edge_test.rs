use egui::pos2;

use crate::toolbar_visible;

#[test]
fn test_the_toolbar_appears_at_the_top_edge() {
    assert!(toolbar_visible(false, Some(pos2(900.0, 2.0)), None), "touching the top edge shows the toolbar");
    assert!(!toolbar_visible(false, Some(pos2(900.0, 40.0)), None), "a pointer below the edge leaves it hidden");
    assert!(!toolbar_visible(true, None, None), "a pointer outside the window hides it");
}
