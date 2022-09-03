use std::process::exit;
use std::fs;

use crate::{colors::{Colors, check_no_color_env}, initialize_colors::initialize_colors};

pub fn write_to_file(path: &str, contents: String) {
    let colors = initialize_colors();

    fs::write(path, contents).unwrap_or_else(|e| {
        println!(
            "{}==>{} Error occurred when writing to {}: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            path,
            e,
            e.kind()
        );
        exit(1);
    });
}
