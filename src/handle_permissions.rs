use std::process::exit;
use crate::initialize_colors;

use crate::GenericMessages;

#[cfg(target_os = "linux")]
use nix::unistd::Uid;

const MESSAGES: GenericMessages = GenericMessages::new();

pub fn handle_permissions() {
    let colors = initialize_colors();

    // Check if the program is running with root permissions
    #[cfg(target_family = "unix")]
    if !Uid::effective().is_root() {
        println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.root_is_required);
        exit(1);
    }

    #[cfg(target_family = "windows")]
    if !is_elevated() {
        println!("{}==>{} {}", colors.bold_red, colors.reset, MESSAGES.root_is_required);
        exit(1);
    }
}
