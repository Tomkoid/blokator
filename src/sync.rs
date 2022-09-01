use std::{process::exit, path::Path};

use crate::{write::write_to_file, get_data_dir, read::read_file_to_string, colors::{Colors, check_no_color_env}};

pub fn sync(repo: &str) {
    let mut colors = Colors::new_without_colors();

    #[cfg(target_family = "unix")]
    if !check_no_color_env() {
        colors = Colors::new();
    }

    let response = ureq::get(repo).call();
    
    let local_hosts = format!(
        "{}/hosts",
        get_data_dir()
    );

    let response = match response {
        Ok(s) => s,
        Err(e) => match e.kind() {
            ureq::ErrorKind::Dns => {
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

    let resp = response.into_string().unwrap_or_else(|e| {
        println!(
            "{}==>{} Error when converting response to String: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            e,
            e.kind()
        );
        exit(1)
    });

    if Path::new(&local_hosts).exists() {
        write_to_file(&local_hosts, read_file_to_string(&local_hosts).unwrap() + &resp + "\n\n");
    } else {
        write_to_file(&local_hosts, resp);
    }
}
