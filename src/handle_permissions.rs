use std::process::exit;

use crate::AppState;

#[cfg(target_family = "unix")]
use nix::unistd::Uid;

#[cfg(target_family = "windows")]
use crate::is_elevated;

pub fn handle_permissions(app_state: &AppState) {
    // Check if the program is running with root permissions
    #[cfg(target_family = "unix")]
    if !Uid::effective().is_root() {
        app_state.logger.log_error("root_is_required");
        exit(1);
    }

    #[cfg(target_family = "windows")]
    if !is_elevated() {
        app_state.logger.log_error("root_is_required");
        exit(1);
    }
}
