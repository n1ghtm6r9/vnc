use std::fs::{self, File};
use std::path::Path;
use std::sync::Mutex;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::LOG_LEVEL_VARIABLE;

pub fn init_logger(log_file: Option<&Path>) {
    let level = std::env::var(LOG_LEVEL_VARIABLE)
        .ok()
        .and_then(|value| value.parse::<LevelFilter>().ok())
        .unwrap_or(LevelFilter::INFO);
    let file = log_file.and_then(|path| {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok()?;
        }
        File::create(path).ok()
    });
    let builder = tracing_subscriber::fmt().with_max_level(level).with_target(false);
    let result = match file {
        Some(file) => builder
            .with_ansi(false)
            .with_writer(std::io::stderr.and(Mutex::new(file)))
            .try_init(),
        None => builder.with_writer(std::io::stderr).try_init(),
    };
    if let Err(error) = result {
        eprintln!("[UI] logging could not start: {error}");
    }
}
