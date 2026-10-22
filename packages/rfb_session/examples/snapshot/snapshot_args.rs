use std::path::PathBuf;

pub(crate) struct SnapshotArgs {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) out: PathBuf,
}
