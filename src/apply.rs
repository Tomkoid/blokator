use std::{process::exit, io::ErrorKind};

use crate::{write::write_to_file, read::read_file_to_string};

pub fn apply(
    path_to_local_hosts: &str,
    path_to_write: &str
    ) {
    
    let contents = match read_file_to_string(path_to_local_hosts) {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!(
                    "==> Local hosts don't exist: {} (Kind: {})",
                    e,
                    e.kind()
                ); 
                println!("Help: try to run the adblocker with `--sync` argument");
                exit(1)
            },
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
    
    write_to_file(path_to_write, contents)
}
