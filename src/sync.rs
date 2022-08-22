use ureq::ErrorKind;
use std::process::exit;

use crate::{write::write_to_file, get_data_dir};

pub fn sync() {
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

    write_to_file(&local_hosts, response.into_string().unwrap())
}
