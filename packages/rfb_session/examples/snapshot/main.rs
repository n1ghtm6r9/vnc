mod connect_options;
mod connect_timeout;
mod encoding_name;
mod first_frame_deadline;
mod parse_args;
mod quiet_wait;
mod run_snapshot;
mod snapshot;
mod snapshot_args;
mod snapshot_draw_patch;
mod snapshot_error;
mod snapshot_handle_event;
mod snapshot_is_complete;
mod snapshot_new;
mod snapshot_on_quiet;
mod snapshot_resize;
mod snapshot_save_png;
mod usage;

use std::{env, process::ExitCode};

pub(crate) use connect_options::*;
pub(crate) use connect_timeout::*;
pub(crate) use encoding_name::*;
pub(crate) use first_frame_deadline::*;
pub(crate) use parse_args::*;
pub(crate) use quiet_wait::*;
pub(crate) use run_snapshot::*;
pub(crate) use snapshot::*;
pub(crate) use snapshot_args::*;
pub(crate) use snapshot_error::*;
pub(crate) use usage::*;

fn main() -> ExitCode {
    let password = env::var("VNC_PASSWORD").unwrap_or_default();
    let Some(args) = parse_args(env::args().skip(1), password) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    match run_snapshot(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("snapshot failed: {error}");
            ExitCode::FAILURE
        }
    }
}
