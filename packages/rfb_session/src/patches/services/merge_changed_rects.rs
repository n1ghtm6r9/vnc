use rfb::Rect;

use super::rects_overlap;
use crate::MAX_PATCHES_PER_UPDATE;

pub(crate) fn merge_changed_rects(changed: &[Rect]) -> Vec<Rect> {
    let changed: Vec<Rect> = changed
        .iter()
        .copied()
        .filter(|rect| !rect.is_empty())
        .collect();
    if changed.len() > MAX_PATCHES_PER_UPDATE {
        return changed
            .into_iter()
            .reduce(|bounds, rect| bounds.union(rect))
            .into_iter()
            .collect();
    }
    let mut merged: Vec<Rect> = Vec::with_capacity(changed.len());
    for rect in changed {
        let mut grown = rect;
        while let Some(index) = merged.iter().position(|&kept| rects_overlap(kept, grown)) {
            grown = grown.union(merged.swap_remove(index));
        }
        merged.push(grown);
    }
    merged
}
