use std::io::ErrorKind;
use std::process::exit;

use crate::{write::write_to_file, read::read_file_to_string};

pub fn restore(
    path_to_backup_file: &str,
    path_to_hosts: &str
    ) {

    let output = match read_file_to_string(path_to_backup_file) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "==> Tried to restore the backup, but it doesn't even exist: {} (Kind: {})",
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
        }
    };

    write_to_file(path_to_hosts, output)
}
