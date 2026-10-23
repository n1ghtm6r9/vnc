#[cfg(windows)]
mod names_from_multi_string;
mod preferred_ui_languages;

#[cfg(windows)]
pub(crate) use names_from_multi_string::*;
pub use preferred_ui_languages::*;
