use egui::{Rect, pos2};

use crate::toolbar_visible;

#[test]
fn test_the_toolbar_hides_once_the_pointer_leaves_it() {
    let toolbar = Rect::from_min_max(pos2(700.0, 6.0), pos2(1500.0, 40.0));
    assert!(toolbar_visible(true, Some(pos2(1000.0, 50.0)), Some(toolbar)), "a pointer just below the toolbar keeps it open");
    assert!(!toolbar_visible(true, Some(pos2(1000.0, 90.0)), Some(toolbar)), "moving well below the toolbar hides it");
}
