use crate::{write::write_to_file, read::read_file_to_string};

pub fn apply(
    path_to_local_hosts: &str,
    path_to_write: &str
    ) {
    
    let contents = match read_file_to_string(path_to_local_hosts) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    };
    
    write_to_file(path_to_write, contents)
}
