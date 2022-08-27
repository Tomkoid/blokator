use std::process::exit;
use std::fs;

use crate::colors::{Colors, check_no_color_env};

pub fn write_to_file(path: &str, contents: String) {
    let mut colors = Colors::new_without_colors();
    
    #[cfg(target_family = "unix")]
    if !check_no_color_env() {
        colors = Colors::new();
    }

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
