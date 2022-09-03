use std::io::ErrorKind;
use std::process::exit;

use crate::initialize_colors::initialize_colors;
use crate::{write::write_to_file, read::read_file_to_string, Actions};
use crate::messages::CopyMessages;

pub fn copy(from: &str, to: &str, action: Actions) {
    let colors = initialize_colors();

    let messages = CopyMessages::new(action);

    let output = match read_file_to_string(from) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "{}==>{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.not_found,
                    e,
                    e.kind()
                );
                exit(1)
            }
            ErrorKind::PermissionDenied => {
                println!(
                    "{}==>{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.permission_denied,
                    e,
                    e.kind()
                );
                exit(1)
            }
            _ => {
                println!(
                    "{}==>{} {}: {} (Kind: {})",
                    colors.bold_red,
                    colors.reset,
                    messages.unknown_error,
                    e,
                    e.kind()
                );
                exit(1)
            }
        }
    };

    write_to_file(to, output)
}
