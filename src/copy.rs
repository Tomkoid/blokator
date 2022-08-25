use std::io::ErrorKind;
use std::process::exit;

use crate::{write::write_to_file, read::read_file_to_string, Actions};
use crate::messages::Messages;

pub fn copy(from: &str, to: &str, action: Actions) {
    let messages = Messages::new(action);

    let output = match read_file_to_string(from) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "==> {}: {} (Kind: {})",
                    messages.not_found,
                    e,
                    e.kind()
                );
                exit(1)
            }
            ErrorKind::PermissionDenied => {
                println!(
                    "==> {}: {} (Kind: {})",
                    messages.permission_denied,
                    e,
                    e.kind()
                );
                exit(1)
            }
            _ => {
                println!(
                    "==> {}: {} (Kind: {})",
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
