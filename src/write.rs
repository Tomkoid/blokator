use std::fs;
use std::process::exit;

use crate::actions::Colors;

pub fn write_to_file(path: &str, contents: String) {
    let colors = Colors::new();

    fs::write(path, contents).unwrap_or_else(|e| {
        eprintln!(
            "{}error:{} Error occurred when writing to {}: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            path,
            e,
            e.kind()
        );
        exit(1);
    });
}
