use std::process::ExitCode;

use super::ClientApp;

impl ClientApp {
    pub fn exit_code(&self) -> ExitCode {
        if self.failed {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
