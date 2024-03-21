use std::fs;
use std::path::Path;
use std::process::exit;

use crate::actions::Colors;
use crate::get_data_dir;
use crate::write::write_to_file;

pub fn already_initialized() -> bool {
    let mut status: bool = true;
    if !Path::new(&(get_data_dir() + "/hosts")).exists() {
        status = false
    };
    if !Path::new(&(get_data_dir() + "/repos")).exists() {
        status = false
    };
    if !Path::new(&get_data_dir()).exists() {
        status = false
    };
    status
}

pub fn initialize_dir() {
    let colors = Colors::new();

    fs::create_dir_all(get_data_dir()).unwrap_or_else(|e| {
        eprintln!(
            "{}error:{} Error occurred when initializing dirs: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            e,
            e.kind()
        );
        exit(1);
    });

    let stevenblack_hosts =
        "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string();
    let repos_file_location = format!("{}/repos", get_data_dir());

    // Create file for repos
    write_to_file(&repos_file_location, stevenblack_hosts);

    let local_hosts_location = format!("{}/hosts", get_data_dir());
    write_to_file(&local_hosts_location, "".to_string())
}
