use rfb_session::{ConnectOptions, Credentials, Quality, encodings_for_quality};

use crate::{CONNECT_TIMEOUT, SnapshotArgs};

pub(crate) fn connect_options(args: &SnapshotArgs) -> ConnectOptions {
    ConnectOptions {
        host: args.host.clone(),
        port: args.port,
        credentials: Credentials::new(&args.user, &args.password),
        shared: true,
        encodings: encodings_for_quality(Quality::Balanced),
        connect_timeout: CONNECT_TIMEOUT,
        apple_remote_desktop: true,
        high_performance: None,
    }
}
