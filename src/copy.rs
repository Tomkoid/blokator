use std::io::ErrorKind;
use std::process::exit;

use crate::initialize_colors::initialize_colors;
use crate::messages::Messages;
use crate::{read::read_file_to_string, write::write_to_file, Actions};

pub fn copy(from: &str, to: &str, action: Actions) {
    let colors = initialize_colors();

    let messages: Messages = Messages::new();

    let not_found_message = match action {
        Actions::Restore => messages.restore_message.get("not_found").unwrap(),
        Actions::Backup => messages.backup_message.get("not_found").unwrap(),
        Actions::Apply => messages.apply_message.get("not_found").unwrap(),
    };

    let output = match read_file_to_string(from) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    not_found_message,
                    e,
                    e.kind()
                );
                exit(1)
            }
            ErrorKind::PermissionDenied => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.message.get("permission_denied").unwrap(),
                    e,
                    e.kind()
                );
                exit(1)
            }
            _ => {
                println!(
                    "{}error:{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.message.get("unknown_error").unwrap(),
                    e,
                    e.kind()
                );
                exit(1)
            }
        },
    };

    write_to_file(to, output)
}
