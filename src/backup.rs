use crate::read::read_file_to_string;
use crate::write::write_to_file;

pub fn backup(file_to_backup: &str, backup_path: &str) {
    let output = match read_file_to_string(file_to_backup) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };

    write_to_file(backup_path, output);
}
