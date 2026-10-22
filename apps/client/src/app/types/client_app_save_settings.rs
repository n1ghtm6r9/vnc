use log::warn;

use super::ClientApp;

impl ClientApp {
    pub(super) fn save_settings(&self) {
        if let Some(path) = &self.settings_path
            && let Err(error) = self.settings.save(path)
        {
            warn!("[UI] settings could not be saved to {}: {error}", path.display());
        }
    }
}
