use ureq::ErrorKind;
use std::{process::exit, path::Path};

use crate::{write::write_to_file, get_data_dir, read::read_file_to_string};

pub fn sync() -> bool {
    let response = ureq::get("https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts").call();
    
    let local_hosts = format!(
        "{}/hosts",
        get_data_dir()
    );

    let response = match response {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ErrorKind::Dns => {
                println!(
                    "==> Connection failed. (Check your internet connection): {} (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            },
            _ => {
                println!(
                    "Error occurred: {} (Kind: {})",
                    e,
                    e.kind()
                );
                exit(1)
            }
        }
    };

    let resp = response.into_string().unwrap();

    let mut changed = true;
    if Path::new(&local_hosts).exists() {
        changed = read_file_to_string(&local_hosts).unwrap() != resp;
    }

    write_to_file(&local_hosts, resp);

    changed
}
