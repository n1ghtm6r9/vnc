use super::TargetOptions;

pub struct ImageOptions<'a> {
    pub source_width: u32,
    pub source_height: u32,
    pub source: &'a [u8],
    pub target: TargetOptions,
}
