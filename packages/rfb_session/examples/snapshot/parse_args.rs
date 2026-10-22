use std::path::PathBuf;

use crate::SnapshotArgs;

pub(crate) fn parse_args(
    mut args: impl Iterator<Item = String>,
    password: String,
) -> Option<SnapshotArgs> {
    let host = args.next()?;
    let port = args.next()?.parse().ok()?;
    let user = args.next()?;
    let out = PathBuf::from(args.next()?);
    if args.next().is_some() {
        return None;
    }
    Some(SnapshotArgs {
        host,
        port,
        user,
        password,
        out,
    })
}
