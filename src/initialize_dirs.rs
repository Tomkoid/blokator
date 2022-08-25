use std::process::exit;
use std::path::Path;
use std::fs;

use crate::{get_data_dir, colors::{check_no_color_env, Colors}};

pub fn already_initialized() -> bool {
    Path::new(&get_data_dir()).exists()
}

pub fn initialize_dir() {
    let colors: Colors;

    if check_no_color_env() {
        colors = Colors::new_without_colors();
    } else {
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
    })
}
