use std::process::exit;
use std::path::Path;
use std::fs;

use crate::get_data_dir;

pub fn already_initialized() -> bool {
    Path::new(&get_data_dir()).exists()
}

pub fn initialize_dir() {
    match fs::create_dir_all(get_data_dir()) {
        Ok(_) => {},
        Err(e) => {
            println!(
                "==> Error occurred when initializing dirs: {} (Kind: {})",
                e,
                e.kind()
            );
            exit(1)
        }
    }
}
