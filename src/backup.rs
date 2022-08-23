use std::io::ErrorKind;
use std::process::exit;

use crate::read::read_file_to_string;
use crate::write::write_to_file;

pub fn backup(file_to_backup: &str, backup_path: &str) {
    let output = match read_file_to_string(file_to_backup) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "==> Tried to backup the /etc/hosts file, but it doesn't even exist: {}, (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            }
            ErrorKind::PermissionDenied => {
                println!(
                    "==> Permission denied: {} (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            }
            _ => {
                println!(
                    "==> Error occurred: {} (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            }
        },
    };

    write_to_file(backup_path, output);
}
