use crate::{write::write_to_file, read::read_file_to_string};

pub fn restore(
    path_to_backup_file: &str,
    path_to_hosts: &str
    ) {

    let output = match read_file_to_string(path_to_backup_file) {
        Ok(s) => s,
        Err(e) => e.to_string()
    };

    write_to_file(path_to_hosts, output)
}
