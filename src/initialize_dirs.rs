use std::process::exit;
use std::path::Path;
use std::fs;

use crate::write::write_to_file;
use crate::{get_data_dir, colors::{check_no_color_env, Colors}};

pub fn already_initialized() -> bool {
    Path::new(&get_data_dir()).exists()
}

pub fn initialize_dir() {
    let mut colors = Colors::new_without_colors();

    #[cfg(target_family = "unix")]
    if !check_no_color_env() {
        colors = Colors::new();
    }

    fs::create_dir_all(get_data_dir()).unwrap_or_else(|e| {
        println!(
            "{}==>{} Error occurred when initializing dirs: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            e,
            e.kind()
        );
        exit(1);
    });

    let stevenblack_hosts = "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string();
    let repos_file_location = format!("{}/repos", get_data_dir());
    
    // Create file for repos
    write_to_file(&repos_file_location, stevenblack_hosts);

    let local_hosts_location = format!("{}/hosts", get_data_dir());
    write_to_file(&local_hosts_location, "".to_string())
}
