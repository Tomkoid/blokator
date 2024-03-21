use std::process::exit;

use crate::{actions::Colors, Messages};

#[cfg(target_family = "unix")]
use nix::unistd::Uid;

#[cfg(target_family = "windows")]
use crate::is_elevated;

pub fn handle_permissions() {
    let colors = Colors::new();
    let messages: Messages = Messages::new();

    // Check if the program is running with root permissions
    #[cfg(target_family = "unix")]
    if !Uid::effective().is_root() {
        println!(
            "{}error:{} {}",
            colors.bold_red,
            colors.reset,
            messages.message.get("root_is_required").unwrap()
        );
        exit(1);
    }

    #[cfg(target_family = "windows")]
    if !is_elevated() {
        println!(
            "{}error:{} {}",
            colors.bold_red,
            colors.reset,
            messages.message.get("root_is_required").unwrap()
        );
        exit(1);
    }
}
