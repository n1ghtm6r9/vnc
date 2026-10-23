use crate::LinkState;

pub fn overlay_hears_keys(link: LinkState) -> bool {
    link != LinkState::Online
}
