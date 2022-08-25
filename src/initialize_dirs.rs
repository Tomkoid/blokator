use std::process::exit;
use std::path::Path;
use std::fs;

use crate::get_data_dir;

pub fn already_initialized() -> bool {
    Path::new(&get_data_dir()).exists()
}

pub fn initialize_dir() {
    fs::create_dir_all(get_data_dir()).unwrap_or_else(|e| {
        println!(
            "==> Error occurred when initializing dirs: {} (Kind: {})",
            e,
            e.kind()
        );
        exit(1);
    })
}
